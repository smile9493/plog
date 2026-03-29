<?php
require __DIR__ . '/vendor/autoload.php';

use Plog\Core\Config\ConfigManager;
use Plog\Core\Config\EnvLoader;

$config = new ConfigManager([new EnvLoader()]);
$config->load(__DIR__ . '/.env');

echo "DB_HOST: " . $config->get('DB_HOST') . "\n";
echo "DB_NAME: " . $config->get('DB_NAME') . "\n";
echo "DB_USER: " . $config->get('DB_USER') . "\n";
