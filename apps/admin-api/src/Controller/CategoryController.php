<?php

declare(strict_types=1);

namespace Plog\AdminApi\Controller;

use Plog\AdminApi\Application;
use Plog\Content\Models\Category;

/**
 * 分类控制器
 */
class CategoryController
{
    /**
     * 应用实例
     *
     * @var Application
     */
    private Application $app;

    /**
     * 分类模型
     *
     * @var Category
     */
    private Category $category;

    /**
     * 构造函数
     *
     * @param Application $app 应用实例
     */
    public function __construct(Application $app)
    {
        $this->app = $app;
        $this->category = new Category($app->getDb());
    }

    /**
     * 获取分类列表
     *
     * @return array
     */
    public function index(): array
    {
        return $this->category->all();
    }

    /**
     * 获取分类详情
     *
     * @param int $id 分类 ID
     * @return array|null
     */
    public function show(int $id): ?array
    {
        return $this->category->find($id);
    }

    /**
     * 创建分类
     *
     * @return int
     */
    public function store(): int
    {
        $data = json_decode(file_get_contents('php://input'), true);

        return $this->category->create($data);
    }

    /**
     * 更新分类
     *
     * @param int $id 分类 ID
     * @return int
     */
    public function update(int $id): int
    {
        $data = json_decode(file_get_contents('php://input'), true);

        return $this->category->update($id, $data);
    }

    /**
     * 删除分类
     *
     * @param int $id 分类 ID
     * @return int
     */
    public function destroy(int $id): int
    {
        return $this->category->delete($id);
    }
}
