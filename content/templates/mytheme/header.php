<?php
/*
Template Name:我的主题
Version:1.0
Template Url:https://www.plog.net/template/
Description:一个现代科技感的Plog主题，采用深色背景和霓虹色设计
Author:CodeArts Agent
Author Url:https://www.plog.net/author/index/1
*/
if(!defined('PLOG_ROOT')) {exit('error!');}
?>
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="keywords" content="<?= $site_key ?>" />
    <meta name="description" content="<?= $site_description ?>" />
    <title><?= $site_title ?></title>
    <link rel="stylesheet" href="<?= TEMPLATE_URL ?>style.css">
    <link rel="stylesheet" href="https://cdn.bootcdn.net/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <?php doAction('index_head') ?>
</head>
<body>
    <!-- 左侧垂直导航栏 -->
    <aside class="sidebar-nav">
        <div class="sidebar-logo">
            <a href="<?= BLOG_URL ?>">
                <i class="fas fa-code"></i>
            </a>
        </div>
        
        <nav class="sidebar-menu">
            <a href="<?= BLOG_URL ?>" class="nav-item active">
                <i class="fas fa-home"></i>
                <span>首页</span>
            </a>
            <?php
            // 输出分类导航
            global $CACHE;
            $sort_cache = $CACHE->readCache('sort');
            $count = 0;
            foreach($sort_cache as $value):
                if($value['pid'] == 0 && $count < 6):
            ?>
            <a href="<?= Url::sort($value['sid']) ?>" class="nav-item">
                <i class="fas fa-folder"></i>
                <span><?= $value['sortname'] ?></span>
            </a>
            <?php
                    $count++;
                endif;
            endforeach;
            ?>
        </nav>
        
        <div class="sidebar-user">
            <i class="fas fa-user-circle"></i>
        </div>
    </aside>
    
    <!-- 主内容区 -->
    <main class="main-content">
