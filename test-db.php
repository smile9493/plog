<?php
try {
    $pdo = new PDO('mysql:host=mysql;dbname=plog', 'plog', 'plog_password');
    echo "Connected successfully to MySQL!\n";
    
    // 测试查询
    $stmt = $pdo->query('SELECT COUNT(*) as count FROM blog');
    $result = $stmt->fetch();
    echo "Blog count: " . $result['count'] . "\n";
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
