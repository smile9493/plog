<?php

declare(strict_types=1);

namespace Plog\Core\Manifest;

/**
 * 插件 Manifest
 */
class PluginManifest
{
    private array $data;
    
    public function __construct(array $data)
    {
        $this->data = $data;
    }
    
    /**
     * 获取插件名称
     */
    public function getName(): string
    {
        return $this->data['name'];
    }
    
    /**
     * 获取插件版本
     */
    public function getVersion(): string
    {
        return $this->data['version'];
    }
    
    /**
     * 获取插件描述
     */
    public function getDescription(): string
    {
        return $this->data['description'];
    }
    
    /**
     * 获取作者信息
     */
    public function getAuthor(): array
    {
        if (is_string($this->data['author'] ?? null)) {
            return ['name' => $this->data['author']];
        }
        
        return $this->data['author'] ?? [];
    }
    
    /**
     * 获取作者名称
     */
    public function getAuthorName(): string
    {
        $author = $this->getAuthor();
        return $author['name'] ?? 'Unknown';
    }
    
    /**
     * 获取许可证
     */
    public function getLicense(): string
    {
        return $this->data['license'] ?? 'Unknown';
    }
    
    /**
     * 获取能力列表
     * 
     * @return string[]
     */
    public function getCapabilities(): array
    {
        return $this->data['capabilities'] ?? [];
    }
    
    /**
     * 检查是否有指定能力
     */
    public function hasCapability(string $capability): bool
    {
        $capabilities = $this->getCapabilities();
        
        // 精确匹配
        if (in_array($capability, $capabilities)) {
            return true;
        }
        
        // 通配符匹配
        $parts = explode(':', $capability);
        $wildcard = $parts[0] . ':*';
        
        return in_array($wildcard, $capabilities);
    }
    
    /**
     * 获取依赖
     */
    public function getDependencies(): array
    {
        return $this->data['dependencies'] ?? [];
    }
    
    /**
     * 获取 Hook 声明
     */
    public function getHooks(): array
    {
        return $this->data['hooks'] ?? [];
    }
    
    /**
     * 获取过滤器
     */
    public function getFilters(): array
    {
        return $this->data['hooks']['filters'] ?? [];
    }
    
    /**
     * 获取动作
     */
    public function getActions(): array
    {
        return $this->data['hooks']['actions'] ?? [];
    }
    
    /**
     * 获取配置声明
     */
    public function getConfig(): array
    {
        return $this->data['config'] ?? [];
    }
    
    /**
     * 获取自动加载配置
     */
    public function getAutoload(): array
    {
        return $this->data['autoload'] ?? [];
    }
    
    /**
     * 获取原始数据
     */
    public function toArray(): array
    {
        return $this->data;
    }
}

/**
 * 主题 Manifest
 */
class ThemeManifest
{
    private array $data;
    
    public function __construct(array $data)
    {
        $this->data = $data;
    }
    
    /**
     * 获取主题名称
     */
    public function getName(): string
    {
        return $this->data['name'];
    }
    
    /**
     * 获取主题版本
     */
    public function getVersion(): string
    {
        return $this->data['version'];
    }
    
    /**
     * 获取主题描述
     */
    public function getDescription(): string
    {
        return $this->data['description'];
    }
    
    /**
     * 获取作者信息
     */
    public function getAuthor(): array
    {
        if (is_string($this->data['author'] ?? null)) {
            return ['name' => $this->data['author']];
        }
        
        return $this->data['author'] ?? [];
    }
    
    /**
     * 获取作者名称
     */
    public function getAuthorName(): string
    {
        $author = $this->getAuthor();
        return $author['name'] ?? 'Unknown';
    }
    
    /**
     * 获取许可证
     */
    public function getLicense(): string
    {
        return $this->data['license'] ?? 'Unknown';
    }
    
    /**
     * 获取模板引擎
     */
    public function getEngine(): string
    {
        return $this->data['engine'];
    }
    
    /**
     * 获取模板列表
     */
    public function getTemplates(): array
    {
        return $this->data['templates'] ?? [];
    }
    
    /**
     * 获取指定模板
     */
    public function getTemplate(string $name): ?array
    {
        return $this->data['templates'][$name] ?? null;
    }
    
    /**
     * 获取模板文件路径
     */
    public function getTemplateFile(string $name): ?string
    {
        $template = $this->getTemplate($name);
        
        if (is_string($template)) {
            return $template;
        }
        
        return $template['file'] ?? null;
    }
    
    /**
     * 检查是否有指定模板
     */
    public function hasTemplate(string $name): bool
    {
        return isset($this->data['templates'][$name]);
    }
    
    /**
     * 获取静态资源
     */
    public function getAssets(): array
    {
        return $this->data['assets'] ?? [];
    }
    
    /**
     * 获取 CSS 文件列表
     * 
     * @return string[]
     */
    public function getCssFiles(): array
    {
        return $this->data['assets']['css'] ?? [];
    }
    
    /**
     * 获取 JS 文件列表
     * 
     * @return string[]
     */
    public function getJsFiles(): array
    {
        return $this->data['assets']['js'] ?? [];
    }
    
    /**
     * 获取支持的特性
     * 
     * @return string[]
     */
    public function getSupports(): array
    {
        return $this->data['supports'] ?? [];
    }
    
    /**
     * 检查是否支持指定特性
     */
    public function supports(string $feature): bool
    {
        return in_array($feature, $this->getSupports());
    }
    
    /**
     * 获取菜单位置
     */
    public function getMenus(): array
    {
        return $this->data['menus'] ?? [];
    }
    
    /**
     * 获取小组件区域
     */
    public function getWidgetAreas(): array
    {
        return $this->data['widgets'] ?? [];
    }
    
    /**
     * 获取自定义选项
     */
    public function getCustomizer(): array
    {
        return $this->data['customizer'] ?? [];
    }
    
    /**
     * 获取依赖
     */
    public function getDependencies(): array
    {
        return $this->data['dependencies'] ?? [];
    }
    
    /**
     * 获取原始数据
     */
    public function toArray(): array
    {
        return $this->data;
    }
}

/**
 * Manifest 异常
 */
class ManifestException extends \Exception
{
}
