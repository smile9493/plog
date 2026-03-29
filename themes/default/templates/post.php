<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?= htmlspecialchars($post['title']) ?> - <?= $site_name ?? 'Plog' ?></title>
    <meta name="description" content="<?= htmlspecialchars($post['excerpt'] ?? '') ?>">
    
    <!-- CSS -->
    <link rel="stylesheet" href="/themes/default/assets/css/style.css">
</head>
<body>
    <!-- Header -->
    <header class="header">
        <div class="container">
            <div class="logo">
                <a href="/"><?= $site_name ?? 'Plog' ?></a>
            </div>
            <nav class="nav">
                <ul>
                    <li><a href="/">首页</a></li>
                    <li><a href="/category">分类</a></li>
                    <li><a href="/archive">归档</a></li>
                    <li><a href="/about">关于</a></li>
                </ul>
            </nav>
        </div>
    </header>

    <!-- Main Content -->
    <main class="main">
        <div class="container">
            <article class="post-detail">
                <header class="post-header">
                    <h1 class="post-title"><?= htmlspecialchars($post['title']) ?></h1>
                    <div class="post-meta">
                        <span class="author">作者: <?= htmlspecialchars($post['author']['username'] ?? 'Admin') ?></span>
                        <span class="date">发布于: <?= date('Y-m-d H:i', strtotime($post['created_at'])) ?></span>
                        <?php if (isset($post['category'])): ?>
                        <span class="category">
                            分类: <a href="/category/<?= $post['category']['id'] ?>"><?= htmlspecialchars($post['category']['name']) ?></a>
                        </span>
                        <?php endif; ?>
                    </div>
                </header>

                <div class="post-content">
                    <?= $post['content'] ?>
                </div>

                <?php if (isset($post['tags']) && !empty($post['tags'])): ?>
                <div class="post-tags">
                    <span>标签:</span>
                    <?php foreach ($post['tags'] as $tag): ?>
                    <a href="/tag/<?= $tag['id'] ?>" class="tag"><?= htmlspecialchars($tag['name']) ?></a>
                    <?php endforeach; ?>
                </div>
                <?php endif; ?>
            </article>

            <!-- 评论区 -->
            <?php if (isset($comments)): ?>
            <section class="comments">
                <h3>评论 (<?= count($comments) ?>)</h3>
                <div class="comment-list">
                    <?php foreach ($comments as $comment): ?>
                    <div class="comment-item">
                        <div class="comment-author">
                            <strong><?= htmlspecialchars($comment['author']) ?></strong>
                            <span class="comment-date"><?= date('Y-m-d H:i', strtotime($comment['created_at'])) ?></span>
                        </div>
                        <div class="comment-content">
                            <?= htmlspecialchars($comment['content']) ?>
                        </div>
                    </div>
                    <?php endforeach; ?>
                </div>
            </section>
            <?php endif; ?>
        </div>
    </main>

    <!-- Footer -->
    <footer class="footer">
        <div class="container">
            <p>&copy; <?= date('Y') ?> <?= $site_name ?? 'Plog' ?>. All rights reserved.</p>
        </div>
    </footer>

    <!-- JavaScript -->
    <script src="/themes/default/assets/js/main.js"></script>
</body>
</html>
