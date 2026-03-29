<?php

declare(strict_types=1);

namespace Plog\Tests\Api;

/**
 * 分类 API 测试
 */
class CategoryApiTest extends ApiTestCase
{
    /**
     * 测试获取分类列表
     */
    public function testGetCategoryList(): void
    {
        $response = $this->get('/categories');

        $this->assertSuccess($response);
        $this->assertArrayHasKey('sorts', $response['body']['data'] ?? []);
    }

    /**
     * 测试分类结构
     */
    public function testCategoryStructure(): void
    {
        $response = $this->get('/categories');

        $this->assertSuccess($response);
        $sorts = $response['body']['data']['sorts'] ?? [];

        if (!empty($sorts)) {
            $firstCategory = reset($sorts);
            $this->assertArrayHasKey('sortname', $firstCategory);
        }
    }

    /**
     * 测试分类层级
     */
    public function testCategoryHierarchy(): void
    {
        $response = $this->get('/categories');

        $this->assertSuccess($response);
        $sorts = $response['body']['data']['sorts'] ?? [];

        // 检查是否有子分类
        foreach ($sorts as $sort) {
            if (isset($sort['children']) && is_array($sort['children'])) {
                $this->assertIsArray($sort['children']);
                break;
            }
        }
    }
}
