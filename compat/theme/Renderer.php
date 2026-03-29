<?php
/**
 * 渲染器
 */

class Renderer
{
    private $engine;
    private $theme;
    private $data = [];
    private $options;
    
    public function __construct(TemplateEngine $engine, array $options = [])
    {
        $this->engine = $engine;
        $this->options = $options;
    }
    
    /**
     * 设置主题
     */
    public function setTheme(array $theme)
    {
        $this->theme = $theme;
        $this->engine->setThemeDir($theme['dir']);
    }
    
    /**
     * 渲染模板
     */
    public function render($template, $data = [])
    {
        if (!$this->theme) {
            throw new \Exception('No theme set');
        }
        
        // 合并数据
        $this->data = array_merge($this->data, $data);
        
        // 获取模板路径
        $templatePath = $this->resolveTemplatePath($template);
        
        if (!$templatePath) {
            throw new \Exception('Template not found: ' . $template);
        }
        
        // 渲染
        return $this->engine->render($templatePath, $this->data);
    }
    
    /**
     * 解析模板路径
     */
    private function resolveTemplatePath($template)
    {
        // 如果是绝对路径
        if (file_exists($template)) {
            return $template;
        }
        
        // 如果是相对路径
        $templatePath = $this->theme['dir'] . '/' . $template;
        
        if (file_exists($templatePath)) {
            return $templatePath;
        }
        
        // 尝试添加 .php 扩展名
        if (!preg_match('/\.php$/', $template)) {
            $templatePath .= '.php';
            if (file_exists($templatePath)) {
                return $templatePath;
            }
        }
        
        return null;
    }
    
    /**
     * 设置数据
     */
    public function setData(array $data)
    {
        $this->data = $data;
    }
    
    /**
     * 获取数据
     */
    public function getData()
    {
        return $this->data;
    }
    
    /**
     * 添加数据
     */
    public function addData(array $data)
    {
        $this->data = array_merge($this->data, $data);
    }
    
    /**
     * 渲染头部
     */
    public function renderHeader($data = [])
    {
        return $this->render('header.php', $data);
    }
    
    /**
     * 渲染尾部
     */
    public function renderFooter($data = [])
    {
        return $this->render('footer.php', $data);
    }
    
    /**
     * 渲染侧边栏
     */
    public function renderSidebar($data = [])
    {
        return $this->render('side.php', $data);
    }
}
