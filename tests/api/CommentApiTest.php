<?php

declare(strict_types=1);

namespace Plog\Tests\Api;

/**
 * 评论 API 测试
 */
class CommentApiTest extends ApiTestCase
{
    /**
     * 测试获取评论列表 - 需要文章 ID
     */
    public function testGetCommentList(): void
    {
        // 先获取一篇文章
        $postResponse = $this->get('/posts', ['count' => 1]);
        $articles = $postResponse['body']['data']['articles'] ?? [];

        if (empty($articles)) {
            $this->markTestSkipped('没有可用的文章');
            return;
        }

        $postId = $articles[0]['id'];
        $response = $this->get('/comments', ['id' => $postId]);

        $this->assertSuccess($response);
    }

    /**
     * 测试获取评论列表 - 缺少文章 ID
     */
    public function testGetCommentListMissingId(): void
    {
        $response = $this->get('/comments');

        $this->assertError($response, 400, 'VALIDATION_ERROR');
    }

    /**
     * 测试获取评论列表 - 带分页
     */
    public function testGetCommentListWithPagination(): void
    {
        // 先获取一篇文章
        $postResponse = $this->get('/posts', ['count' => 1]);
        $articles = $postResponse['body']['data']['articles'] ?? [];

        if (empty($articles)) {
            $this->markTestSkipped('没有可用的文章');
            return;
        }

        $postId = $articles[0]['id'];
        $response = $this->get('/comments', [
            'id' => $postId,
            'page' => 1,
        ]);

        $this->assertSuccess($response);
    }

    /**
     * 测试简化版评论列表
     */
    public function testGetCommentListSimple(): void
    {
        // 先获取一篇文章
        $postResponse = $this->get('/posts', ['count' => 1]);
        $articles = $postResponse['body']['data']['articles'] ?? [];

        if (empty($articles)) {
            $this->markTestSkipped('没有可用的文章');
            return;
        }

        $postId = $articles[0]['id'];
        $response = $this->get('/comment/list/simple', ['id' => $postId]);

        $this->assertSuccess($response);
        $this->assertArrayHasKey('comments', $response['body']['data'] ?? []);
    }
}
