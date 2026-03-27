<?php

/**
 * @package PLOG
 * @link https://www.plog.net
 */

require_once 'init.php';

$emDispatcher = Dispatcher::getInstance();
$emDispatcher->dispatch();
View::output();
