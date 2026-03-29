<?php

declare(strict_types=1);

namespace Plog\Core\Manifest;

/**
 * Manifest 解析器
 * 
 * 用于解析插件和主题的 manifest.json 文件
 */
class ManifestParser
{
    /**
     * 解析插件 manifest
     */
    public function parsePlugin(string $pluginPath): PluginManifest
    {
        $manifestFile = rtrim($pluginPath, '/') . '/manifest.json';
        
        if (!file_exists($manifestFile)) {
            throw new ManifestException("Plugin manifest not found: {$manifestFile}");
        }
        
        $content = file_get_contents($manifestFile);
        $data = json_decode($content, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new ManifestException("Invalid JSON in manifest: " . json_last_error_msg());
        }
        
        $this->validatePluginManifest($data, $manifestFile);
        
        return new PluginManifest($data);
    }
    
    /**
     * 解析主题 manifest
     */
    public function parseTheme(string $themePath): ThemeManifest
    {
        $manifestFile = rtrim($themePath, '/') . '/manifest.json';
        
        if (!file_exists($manifestFile)) {
            throw new ManifestException("Theme manifest not found: {$manifestFile}");
        }
        
        $content = file_get_contents($manifestFile);
        $data = json_decode($content, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new ManifestException("Invalid JSON in manifest: " . json_last_error_msg());
        }
        
        $this->validateThemeManifest($data, $manifestFile);
        
        return new ThemeManifest($data);
    }
    
    /**
     * 验证插件 manifest
     */
    private function validatePluginManifest(array $data, string $file): void
    {
        $required = ['name', 'version', 'description', 'capabilities'];
        
        foreach ($required as $field) {
            if (!isset($data[$field])) {
                throw new ManifestException("Required field '{$field}' missing in {$file}");
            }
        }
        
        // 验证版本格式
        if (!preg_match('/^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$/', $data['version'])) {
            throw new ManifestException("Invalid version format in {$file}: {$data['version']}");
        }
        
        // 验证能力声明
        if (!is_array($data['capabilities'])) {
            throw new ManifestException("Capabilities must be an array in {$file}");
        }
        
        // 验证能力格式
        foreach ($data['capabilities'] as $capability) {
            if (!preg_match('/^[a-z]+:[a-z]+$|^[a-z]+:\*$/', $capability)) {
                throw new ManifestException("Invalid capability format in {$file}: {$capability}");
            }
        }
    }
    
    /**
     * 验证主题 manifest
     */
    private function validateThemeManifest(array $data, string $file): void
    {
        $required = ['name', 'version', 'description', 'engine', 'templates'];
        
        foreach ($required as $field) {
            if (!isset($data[$field])) {
                throw new ManifestException("Required field '{$field}' missing in {$file}");
            }
        }
        
        // 验证版本格式
        if (!preg_match('/^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$/', $data['version'])) {
            throw new ManifestException("Invalid version format in {$file}: {$data['version']}");
        }
        
        // 验证模板声明
        if (!is_array($data['templates'])) {
            throw new ManifestException("Templates must be an object in {$file}");
        }
        
        // 验证模板引擎
        $supportedEngines = ['blade', 'twig', 'php', 'smarty'];
        if (!in_array($data['engine'], $supportedEngines)) {
            throw new ManifestException("Unsupported template engine in {$file}: {$data['engine']}");
        }
    }
    
    /**
     * 扫描插件目录
     * 
     * @return PluginManifest[]
     */
    public function scanPlugins(string $pluginsDir): array
    {
        $plugins = [];
        
        if (!is_dir($pluginsDir)) {
            return $plugins;
        }
        
        $dirs = scandir($pluginsDir);
        
        foreach ($dirs as $dir) {
            if ($dir === '.' || $dir === '..') {
                continue;
            }
            
            $path = $pluginsDir . '/' . $dir;
            
            if (!is_dir($path)) {
                continue;
            }
            
            if (!file_exists($path . '/manifest.json')) {
                continue;
            }
            
            try {
                $plugins[$dir] = $this->parsePlugin($path);
            } catch (ManifestException $e) {
                // 记录错误但继续扫描
                error_log("Failed to parse plugin manifest for {$dir}: " . $e->getMessage());
            }
        }
        
        return $plugins;
    }
    
    /**
     * 扫描主题目录
     * 
     * @return ThemeManifest[]
     */
    public function scanThemes(string $themesDir): array
    {
        $themes = [];
        
        if (!is_dir($themesDir)) {
            return $themes;
        }
        
        $dirs = scandir($themesDir);
        
        foreach ($dirs as $dir) {
            if ($dir === '.' || $dir === '..') {
                continue;
            }
            
            $path = $themesDir . '/' . $dir;
            
            if (!is_dir($path)) {
                continue;
            }
            
            if (!file_exists($path . '/manifest.json')) {
                continue;
            }
            
            try {
                $themes[$dir] = $this->parseTheme($path);
            } catch (ManifestException $e) {
                // 记录错误但继续扫描
                error_log("Failed to parse theme manifest for {$dir}: " . $e->getMessage());
            }
        }
        
        return $themes;
    }
}
