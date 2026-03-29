<?php

declare(strict_types=1);

namespace Plog\Tests\Unit;

use PHPUnit\Framework\TestCase;
use Plog\Core\Event\EventDispatcher;

/**
 * EventDispatcher 测试
 */
class EventDispatcherTest extends TestCase
{
    /**
     * 测试事件监听和触发
     */
    public function testEventListenAndDispatch(): void
    {
        $dispatcher = new EventDispatcher();
        $called = false;

        $dispatcher->listen('test.event', function () use (&$called) {
            $called = true;
        });

        $dispatcher->dispatch('test.event');

        $this->assertTrue($called);
    }

    /**
     * 测试事件参数传递
     */
    public function testEventWithPayload(): void
    {
        $dispatcher = new EventDispatcher();
        $receivedData = null;

        $dispatcher->listen('test.event', function ($data) use (&$receivedData) {
            $receivedData = $data;
        });

        $dispatcher->dispatch('test.event', ['message' => 'Hello']);

        $this->assertEquals(['message' => 'Hello'], $receivedData);
    }

    /**
     * 测试事件优先级
     */
    public function testEventPriority(): void
    {
        $dispatcher = new EventDispatcher();
        $order = [];

        $dispatcher->listen('test.event', function () use (&$order) {
            $order[] = 'second';
        }, 0);

        $dispatcher->listen('test.event', function () use (&$order) {
            $order[] = 'first';
        }, 10);

        $dispatcher->dispatch('test.event');

        $this->assertEquals(['first', 'second'], $order);
    }

    /**
     * 测试移除监听器
     */
    public function testForgetEvent(): void
    {
        $dispatcher = new EventDispatcher();

        $dispatcher->listen('test.event', function () {
            return 'called';
        });

        $this->assertTrue($dispatcher->hasListeners('test.event'));

        $dispatcher->forget('test.event');

        $this->assertFalse($dispatcher->hasListeners('test.event'));
    }
}
