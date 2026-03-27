<?php
if(!defined('PLOG_ROOT')) {exit('error!');}
require_once View::getView('header');
?>

<div class="container">
    <article class="post-single">
        <header class="post-header">
            <h1 class="post-title"><?= $log_title ?></h1>
            <div class="post-meta">
                <time datetime="<?= date('c', $date) ?>"><?= date('Y-m-d', $date) ?></time>
                <span class="author">作者：<?= blog_author($author) ?></span>
                <span class="category">分类：<?= blog_sort($logid) ?></span>
                <span class="views"><?= $views ?> 阅读</span>
                <span class="comments"><?= $comnum ?> 评论</span>
            </div>
        </header>
        
        <?php if($log_cover): ?>
        <div class="post-thumbnail">
            <img src="<?= $log_cover ?>" alt="<?= $log_title ?>">
        </div>
        <?php endif; ?>
        
        <div class="post-content">
            <?= $log_content ?>
        </div>
        
        <footer class="post-footer">
            <div class="post-tags">
                <?= blog_tag($logid) ?>
            </div>
            <?= editflg($logid, $author) ?>
        </footer>
        
        <?php doAction('log_related', $logData) ?>
        
        <div class="post-navigation">
            <?= neighbor_log($neighborLog) ?>
        </div>
        
        <section class="comments-section">
            <h3>评论</h3>
            <?= blog_comments($comments) ?>
            <?= blog_comments_post($logid, $ckname, $ckmail, $ckurl, $verifyCode, $allow_remark) ?>
        </section>
    </article>
</div>

<?php
require_once View::getView('footer');
?>
