<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 主题接口
 * 
 * 定义主题的基本属性和行为
 */
interface ThemeInterface
{
    /**
     * 获取主题名称
     */
    public function getName(): string;

    /**
     * 获取主题版本
     */
    public function getVersion(): string;

    /**
     * 获取主题描述
     */
    public function getDescription(): string;

    /**
     * 获取主题作者
     */
    public function getAuthor(): string;

    /**
     * 获取主题标识
     */
    public function getSlug(): string;

    /**
     * 获取主题目录
     */
    public function getPath(): string;

    /**
     * 获取主题 URL
     */
    public function getUrl(): string;

    /**
     * 获取模板引擎
     */
    public function getEngine(): string;

    /**
     * 获取主题截图
     */
    public function getScreenshot(): string;

    /**
     * 获取主题配置
     */
    public function getConfig(): array;

    /**
     * 获取支持的特性
     * 
     * @return string[]
     */
    public function getSupports(): array;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 模板接口
 * 
 * 定义模板的基本属性和行为
 */
interface TemplateInterface
{
    /**
     * 获取模板名称
     */
    public function getName(): string;

    /**
     * 获取模板文件路径
     */
    public function getPath(): string;

    /**
     * 获取模板类型
     */
    public function getType(): string;

    /**
     * 渲染模板
     */
    public function render(array $data = []): string;

    /**
     * 模板是否存在
     */
    public function exists(): bool;
}

/**
 * 渲染管线接口
 * 
 * 定义渲染流程的抽象层
 */
interface RenderPipelineInterface
{
    /**
     * 设置主题
     */
    public function setTheme(ThemeInterface $theme): void;

    /**
     * 获取当前主题
     */
    public function getTheme(): ThemeInterface;

    /**
     * 渲染页面
     */
    public function render(string $template, array $data = []): string;

    /**
     * 渲染文章
     */
    public function renderPost(PostInterface $post): string;

    /**
     * 渲染分类列表
     */
    public function renderCategoryList(CategoryInterface $category, array $posts): string;

    /**
     * 渲染标签列表
     */
    public function renderTagList(TagInterface $tag, array $posts): string;

    /**
     * 渲染搜索结果
     */
    public function renderSearchResults(string $keyword, array $posts): string;

    /**
     * 添加渲染前钩子
     */
    public function addBeforeRenderHook(callable $hook): void;

    /**
     * 添加渲染后钩子
     */
    public function addAfterRenderHook(callable $hook): void;
}
