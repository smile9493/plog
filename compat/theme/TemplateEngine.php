<?php
/**
 * 模板引擎
 * 
 * 支持 PHP 原生模板和 Tera 模板转换
 */

class TemplateEngine
{
    private $themeDir;
    private $cache = [];
    private $rustApiUrl;
    
    public function __construct(array $options = [])
    {
        $this->rustApiUrl = $options['rust_api_url'] ?? 'http://127.0.0.1:8080';
    }
    
    /**
     * 设置主题目录
     */
    public function setThemeDir($themeDir)
    {
        $this->themeDir = $themeDir;
    }
    
    /**
     * 渲染模板
     */
    public function render($templatePath, $data = [])
    {
        // 检查是否是 Tera 模板
        if ($this->isTeraTemplate($templatePath)) {
            return $this->renderTera($templatePath, $data);
        }
        
        // PHP 原生模板
        return $this->renderPhp($templatePath, $data);
    }
    
    /**
     * 渲染 PHP 模板
     */
    private function renderPhp($templatePath, $data = [])
    {
        // 提取变量到当前作用域
        extract($data);
        
        // 开始输出缓冲
        ob_start();
        
        try {
            include $templatePath;
            $content = ob_get_contents();
        } catch (\Exception $e) {
            ob_end_clean();
            throw $e;
        }
        
        ob_end_clean();
        
        return $content;
    }
    
    /**
     * 渲染 Tera 模板
     */
    private function renderTera($templatePath, $data = [])
    {
        // 读取模板内容
        $templateContent = file_get_contents($templatePath);
        
        // 转换 PHP 语法到 Tera
        $teraContent = $this->convertPhpToTera($templateContent);
        
        // 调用 Rust API 渲染
        return $this->callRustRenderer($teraContent, $data);
    }
    
    /**
     * 检查是否是 Tera 模板
     */
    private function isTeraTemplate($templatePath)
    {
        return preg_match('/\.tera$/', $templatePath);
    }
    
    /**
     * 转换 PHP 语法到 Tera
     */
    public function convertPhpToTera($phpContent)
    {
        $teraContent = $phpContent;
        
        // 转换变量
        // <?php echo $var; ?> → {{ var }}
        $teraContent = preg_replace(
            '/<\?php\s+echo\s+\$([a-zA-Z_][a-zA-Z0-9_]*);?\s*\?>/',
            '{{ $1 }}',
            $teraContent
        );
        
        // <?= $var ?> → {{ var }}
        $teraContent = preg_replace(
            '/<\?=\s*\$([a-zA-Z_][a-zA-Z0-9_]*);?\s*\?>/',
            '{{ $1 }}',
            $teraContent
        );
        
        // 转换 if 语句
        // <?php if ($cond): ?> → {% if cond %}
        $teraContent = preg_replace(
            '/<\?php\s+if\s+\(([^)]+)\):\s*\?>/',
            '{% if $1 %}',
            $teraContent
        );
        
        // <?php else: ?> → {% else %}
        $teraContent = preg_replace(
            '/<\?php\s+else:\s*\?>/',
            '{% else %}',
            $teraContent
        );
        
        // <?php endif; ?> → {% endif %}
        $teraContent = preg_replace(
            '/<\?php\s+endif;\s*\?>/',
            '{% endif %}',
            $teraContent
        );
        
        // 转换 foreach 语句
        // <?php foreach ($items as $item): ?> → {% for item in items %}
        $teraContent = preg_replace(
            '/<\?php\s+foreach\s+\((\$\w+)\s+as\s+\$(\w+)\):\s*\?>/',
            '{% for $2 in $1 %}',
            $teraContent
        );
        
        // <?php endforeach; ?> → {% endfor %}
        $teraContent = preg_replace(
            '/<\?php\s+endforeach;\s*\?>/',
            '{% endfor %}',
            $teraContent
        );
        
        return $teraContent;
    }
    
    /**
     * 调用 Rust 渲染器
     */
    private function callRustRenderer($template, $data)
    {
        try {
            $url = $this->rustApiUrl . '/api/v2/render';
            
            $payload = json_encode([
                'template' => $template,
                'data' => $data,
            ]);
            
            $ch = curl_init();
            curl_setopt_array($ch, [
                CURLOPT_URL => $url,
                CURLOPT_POST => true,
                CURLOPT_POSTFIELDS => $payload,
                CURLOPT_RETURNTRANSFER => true,
                CURLOPT_TIMEOUT => 10,
                CURLOPT_HTTPHEADER => [
                    'Content-Type: application/json',
                ],
            ]);
            
            $response = curl_exec($ch);
            $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
            curl_close($ch);
            
            if ($status === 200) {
                $result = json_decode($response, true);
                return $result['html'] ?? $response;
            }
            
            // 如果 Rust 渲染失败，回退到 PHP 渲染
            return $this->renderPhpFromString($template, $data);
            
        } catch (\Exception $e) {
            // 回退到 PHP 渲染
            return $this->renderPhpFromString($template, $data);
        }
    }
    
    /**
     * 从字符串渲染 PHP 模板
     */
    private function renderPhpFromString($template, $data = [])
    {
        extract($data);
        
        ob_start();
        eval('?>' . $template);
        $content = ob_get_contents();
        ob_end_clean();
        
        return $content;
    }
    
    /**
     * 解析模板路径
     */
    public function resolvePath($template)
    {
        // 如果是绝对路径
        if (file_exists($template)) {
            return $template;
        }
        
        // 如果是相对路径
        $templatePath = $this->themeDir . '/' . $template;
        
        if (file_exists($templatePath)) {
            return $templatePath;
        }
        
        // 尝试添加扩展名
        $extensions = ['.php', '.tera'];
        
        foreach ($extensions as $ext) {
            $path = $templatePath . $ext;
            if (file_exists($path)) {
                return $path;
            }
        }
        
        return null;
    }
}
