<?php

declare(strict_types=1);

namespace Plog\AdminApi\Controller;

use Plog\AdminApi\Application;
use Plog\Content\Models\Post;

/**
 * 文章控制器
 */
class PostController
{
    /**
     * 应用实例
     *
     * @var Application
     */
    private Application $app;

    /**
     * 文章模型
     *
     * @var Post
     */
    private Post $post;

    /**
     * 构造函数
     *
     * @param Application $app 应用实例
     */
    public function __construct(Application $app)
    {
        $this->app = $app;
        $this->post = new Post($app->getDb());
    }

    /**
     * 获取文章列表
     *
     * @return array
     */
    public function index(): array
    {
        $page = (int) ($_GET['page'] ?? 1);
        $perPage = (int) ($_GET['per_page'] ?? 20);

        return $this->post->paginate($page, $perPage);
    }

    /**
     * 获取文章详情
     *
     * @param int $id 文章 ID
     * @return array|null
     */
    public function show(int $id): ?array
    {
        return $this->post->find($id);
    }

    /**
     * 创建文章
     *
     * @return int
     */
    public function store(): int
    {
        $data = json_decode(file_get_contents('php://input'), true);

        return $this->post->create($data);
    }

    /**
     * 更新文章
     *
     * @param int $id 文章 ID
     * @return int
     */
    public function update(int $id): int
    {
        $data = json_decode(file_get_contents('php://input'), true);

        return $this->post->update($id, $data);
    }

    /**
     * 删除文章
     *
     * @param int $id 文章 ID
     * @return int
     */
    public function destroy(int $id): int
    {
        return $this->post->delete($id);
    }
}
