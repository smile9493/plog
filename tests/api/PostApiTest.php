<?php

declare(strict_types=1);

namespace Plog\Tests\Api;

/**
 * 文章 API 测试
 */
class PostApiTest extends ApiTestCase
{
    /**
     * 测试获取文章列表
     */
    public function testGetPostList(): void
    {
        $response = $this->get('/posts');

        $this->assertSuccess($response);
        $this->assertArrayHasKey('articles', $response['body']['data'] ?? []);
        $this->assertArrayHasKey('page', $response['body']['data'] ?? []);
        $this->assertArrayHasKey('total_pages', $response['body']['data'] ?? []);
    }

    /**
     * 测试获取文章列表 - 带分页
     */
    public function testGetPostListWithPagination(): void
    {
        $response = $this->get('/posts', [
            'page' => 1,
            'count' => 10,
        ]);

        $this->assertSuccess($response);
        $this->assertEquals(1, $response['body']['data']['page'] ?? 0);
    }

    /**
     * 测试获取文章列表 - 带分类筛选
     */
    public function testGetPostListWithCategory(): void
    {
        $response = $this->get('/posts', [
            'sort_id' => 1,
        ]);

        $this->assertSuccess($response);
    }

    /**
     * 测试获取文章列表 - 带搜索
     */
    public function testGetPostListWithSearch(): void
    {
        $response = $this->get('/posts', [
            'keyword' => '测试',
        ]);

        $this->assertSuccess($response);
    }

    /**
     * 测试获取文章详情
     */
    public function testGetPostDetail(): void
    {
        // 先获取文章列表
        $listResponse = $this->get('/posts', ['count' => 1]);
        $articles = $listResponse['body']['data']['articles'] ?? [];

        if (empty($articles)) {
            $this->markTestSkipped('没有可用的文章');
            return;
        }

        $postId = $articles[0]['id'];
        $response = $this->get("/posts/{$postId}");

        $this->assertSuccess($response);
        $this->assertArrayHasKey('article', $response['body']['data'] ?? []);
    }

    /**
     * 测试获取文章详情 - 文章不存在
     */
    public function testGetPostDetailNotFound(): void
    {
        $response = $this->get('/posts/999999');
        $this->assertError($response, 404, 'RESOURCE_NOT_FOUND');
    }

    /**
     * 测试创建文章
     */
    public function testCreatePost(): void
    {
        // 需要先登录
        $loginResponse = $this->post('/auth/login', [
            'username' => 'admin',
            'password' => 'password123',
        ]);

        $token = $loginResponse['body']['data']['token'] ?? '';
        if (empty($token)) {
            $this->markTestSkipped('登录失败');
            return;
        }

        $this->setToken($token);

        $response = $this->post('/posts', [
            'title' => '测试文章标题 - ' . time(),
            'content' => '<p>这是测试文章内容</p>',
            'excerpt' => '测试文章摘要',
            'sort_id' => 1,
            'tags' => '测试,API',
        ]);

        $this->assertSuccess($response);
        $this->assertArrayHasKey('article_id', $response['body']['data'] ?? []);
    }

    /**
     * 测试创建文章 - 未授权
     */
    public function testCreatePostUnauthorized(): void
    {
        $response = $this->post('/posts', [
            'title' => '测试文章标题',
            'content' => '测试文章内容',
        ]);

        $this->assertError($response, 401, 'AUTH_REQUIRED');
    }

    /**
     * 测试创建文章 - 缺少必填字段
     */
    public function testCreatePostMissingFields(): void
    {
        // 需要先登录
        $loginResponse = $this->post('/auth/login', [
            'username' => 'admin',
            'password' => 'password123',
        ]);

        $token = $loginResponse['body']['data']['token'] ?? '';
        $this->setToken($token);

        $response = $this->post('/posts', [
            'excerpt' => '只有摘要没有标题',
        ]);

        $this->assertError($response, 422, 'VALIDATION_ERROR');
    }

    /**
     * 测试排序
     */
    public function testPostListOrdering(): void
    {
        // 按浏览量排序
        $response = $this->get('/posts', [
            'order' => 'views',
            'count' => 5,
        ]);

        $this->assertSuccess($response);
        $articles = $response['body']['data']['articles'] ?? [];

        if (count($articles) >= 2) {
            // 验证排序
            $views = array_column($articles, 'views');
            $this->assertEquals($views, array_reverse(sort(array_reverse($views))));
        }
    }
}
