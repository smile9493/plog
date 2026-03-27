#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Emlog主题同步工具
通过SSH+SCP实现主题文件同步和Docker容器操作
"""

import os
import sys
import json
import argparse
import subprocess
from pathlib import Path
from datetime import datetime

# 修复Windows控制台编码问题
if sys.platform == 'win32':
    import io
    import locale
    # 设置环境变量强制使用UTF-8
    os.environ['PYTHONIOENCODING'] = 'utf-8'
    # 重新包装stdout和stderr
    if sys.stdout.encoding != 'utf-8':
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    if sys.stderr.encoding != 'utf-8':
        sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')


class EmlogThemeSync:
    """Emlog主题同步管理类"""

    def __init__(self, config_file='../config/sync_config.json'):
        self.config = self.load_config(config_file)
        self.server = self.config['server']
        self.theme_name = self.config['theme']['name']
        self.local_path = Path(self.config['theme']['local_path'])
        self.remote_path = self.config['theme']['remote_path']

    def load_config(self, config_file):
        """加载配置文件"""
        config_path = Path(__file__).parent / config_file
        if not config_path.exists():
            print(f"❌ 配置文件不存在: {config_file}")
            print("请先创建配置文件,参考 config/sync_config.json.example")
            sys.exit(1)

        with open(config_path, 'r', encoding='utf-8') as f:
            return json.load(f)

    def run_ssh_command(self, command, show_output=True):
        """执行SSH命令"""
        ssh_cmd = [
            'ssh',
            '-o', 'StrictHostKeyChecking=no',
            '-o', 'BatchMode=yes',
            '-o', 'ConnectTimeout=10',
            f"{self.server['user']}@{self.server['host']}",
            command
        ]

        try:
            # 使用UTF-8编码处理输出
            result = subprocess.run(
                ssh_cmd,
                capture_output=True,
                timeout=30
            )

            # 解码输出
            stdout_text = result.stdout.decode('utf-8', errors='replace').strip() if result.stdout else ""
            stderr_text = result.stderr.decode('utf-8', errors='replace').strip() if result.stderr else ""

            # SSH警告信息在stderr中,但命令执行成功
            if result.returncode != 0 and not stdout_text:
                if show_output:
                    print(f"❌ SSH命令执行失败: {command}")
                    if stderr_text and 'Warning' not in stderr_text:
                        print(f"错误: {stderr_text}")
                return None

            if show_output and stdout_text:
                print(stdout_text)
            return stdout_text

        except subprocess.TimeoutExpired:
            print(f"❌ SSH命令超时: {command}")
            return None
        except Exception as e:
            print(f"❌ SSH命令异常: {e}")
            return None

    def test_connection(self):
        """测试SSH连接"""
        print(f"🔍 测试SSH连接: {self.server['user']}@{self.server['host']}")
        result = self.run_ssh_command("echo 'SSH_CONNECTION_OK' && hostname", show_output=False)

        if result and 'SSH_CONNECTION_OK' in result:
            print(f"✅ SSH连接成功")
            return True
        else:
            print(f"❌ SSH连接失败,请检查密钥配置")
            return False

    def check_docker_status(self):
        """检查Docker容器状态"""
        print(f"\n🔍 检查emlog容器状态...")

        # 查找emlog容器
        cmd = f"docker ps --filter 'name={self.config['docker']['container_name']}' --format '{{{{.Names}}}}\t{{{{.Status}}}}\t{{{{.Ports}}}}'"
        result = self.run_ssh_command(cmd, show_output=False)

        if result:
            print(f"✅ 找到emlog容器:")
            print(result)
            return True
        else:
            print(f"❌ 未找到emlog容器: {self.config['docker']['container_name']}")
            return False

    def get_theme_path(self):
        """获取主题目录路径"""
        print(f"\n🔍 查找主题目录...")

        # 检查是否配置了固定路径
        if self.remote_path:
            print(f"✅ 使用配置的主题路径: {self.remote_path}")
            return self.remote_path

        # 动态查找容器挂载点
        cmd = f"docker inspect {self.config['docker']['container_name']} --format='{{{{range .Mounts}}}}{{{{if eq .Destination \"/app\"}}}}{{{{.Source}}}}{{{{end}}}}{{{{end}}}}'"
        result = self.run_ssh_command(cmd, show_output=False)

        if result:
            theme_path = f"{result}/content/templates/{self.theme_name}"
            print(f"✅ 找到主题路径: {theme_path}")
            return theme_path
        else:
            print(f"❌ 无法确定主题路径")
            return None

    def sync_theme(self):
        """同步主题文件"""
        print(f"\n📤 开始同步主题文件...")
        print(f"   本地路径: {self.local_path}")
        print(f"   主题名称: {self.theme_name}")

        # 测试连接
        if not self.test_connection():
            return False

        # 获取主题路径
        theme_path = self.get_theme_path()
        if not theme_path:
            return False

        # 创建目标目录
        print(f"\n📁 创建目标目录...")
        self.run_ssh_command(f"mkdir -p {theme_path}")

        # 同步文件
        print(f"\n📤 同步主题文件...")
        local_theme_path = self.local_path / self.theme_name

        if not local_theme_path.exists():
            print(f"❌ 本地主题目录不存在: {local_theme_path}")
            return False

        # 使用SCP同步
        scp_cmd = [
            'scp', '-r',
            '-o', 'StrictHostKeyChecking=no',
        ]

        # 添加所有主题文件
        theme_files = list(local_theme_path.glob('*'))
        scp_cmd.extend([str(f) for f in theme_files])
        scp_cmd.append(f"{self.server['user']}@{self.server['host']}:{theme_path}/")

        try:
            print(f"   同步文件: {[f.name for f in theme_files]}")
            result = subprocess.run(scp_cmd, capture_output=True, text=True, timeout=60)

            if result.returncode != 0:
                print(f"❌ SCP同步失败: {result.stderr}")
                return False

            print(f"✅ 主题文件同步成功")

            # 修正文件权限
            print(f"\n🔐 修正文件权限...")
            self.run_ssh_command(f"chown -R {self.config['docker']['file_owner']} {theme_path}")
            self.run_ssh_command(f"chmod -R 755 {theme_path}")

            return True

        except Exception as e:
            print(f"❌ 同步异常: {e}")
            return False

    def verify_sync(self):
        """验证同步结果"""
        print(f"\n🔍 验证主题文件...")

        theme_path = self.get_theme_path()
        if not theme_path:
            return False

        # 列出远程主题文件
        result = self.run_ssh_command(f"ls -lh {theme_path}/", show_output=False)

        if result:
            print(f"✅ 主题文件列表:")
            print(result)
            return True
        else:
            print(f"❌ 验证失败")
            return False

    def docker_restart(self):
        """重启Docker容器"""
        print(f"\n🔄 重启emlog容器...")
        result = self.run_ssh_command(f"docker restart {self.config['docker']['container_name']}")

        if result:
            print(f"✅ 容器重启成功")
            return True
        else:
            print(f"❌ 容器重启失败")
            return False

    def docker_logs(self, lines=50):
        """查看Docker日志"""
        print(f"\n📋 查看容器日志(最近{lines}行)...")
        self.run_ssh_command(f"docker logs {self.config['docker']['container_name']} --tail {lines}")

    def backup_theme(self):
        """备份远程主题"""
        print(f"\n💾 备份远程主题...")

        theme_path = self.get_theme_path()
        if not theme_path:
            return False

        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        backup_file = f"/tmp/{self.theme_name}_backup_{timestamp}.tar.gz"

        result = self.run_ssh_command(f"tar -czf {backup_file} -C {theme_path} . && echo '备份文件: {backup_file}'")

        if result:
            print(f"✅ 主题备份成功: {backup_file}")
            return True
        else:
            print(f"❌ 主题备份失败")
            return False

    def full_sync(self):
        """完整同步流程"""
        print("=" * 60)
        print("🚀 Emlog主题完整同步流程")
        print("=" * 60)

        # 1. 测试连接
        if not self.test_connection():
            return False

        # 2. 检查Docker状态
        if not self.check_docker_status():
            return False

        # 3. 备份远程主题
        self.backup_theme()

        # 4. 同步主题
        if not self.sync_theme():
            return False

        # 5. 验证同步
        if not self.verify_sync():
            return False

        print("\n" + "=" * 60)
        print("✅ 主题同步完成!")
        print("=" * 60)
        print(f"\n🌐 访问地址:")
        print(f"   前台: http://{self.server['host']}:{self.config['docker']['web_port']}")
        print(f"   后台: http://{self.server['host']}:{self.config['docker']['web_port']}/admin")
        print(f"\n📝 下一步:")
        print(f"   1. 登录后台")
        print(f"   2. 进入'外观' -> '模板管理'")
        print(f"   3. 启用 '{self.theme_name}' 主题")

        return True


def main():
    """主函数"""
    parser = argparse.ArgumentParser(description='Emlog主题同步工具')
    parser.add_argument('action', choices=['sync', 'verify', 'restart', 'logs', 'backup', 'full'],
                       help='执行的操作: sync=同步, verify=验证, restart=重启容器, logs=查看日志, backup=备份, full=完整流程')
    parser.add_argument('-c', '--config', default='../config/sync_config.json',
                       help='配置文件路径 (默认: ../config/sync_config.json)')
    parser.add_argument('-n', '--lines', type=int, default=50,
                       help='查看日志的行数 (默认: 50)')

    args = parser.parse_args()

    # 创建同步对象
    sync = EmlogThemeSync(args.config)

    # 执行操作
    if args.action == 'sync':
        sync.sync_theme()
    elif args.action == 'verify':
        sync.verify_sync()
    elif args.action == 'restart':
        sync.docker_restart()
    elif args.action == 'logs':
        sync.docker_logs(args.lines)
    elif args.action == 'backup':
        sync.backup_theme()
    elif args.action == 'full':
        sync.full_sync()


if __name__ == '__main__':
    main()
