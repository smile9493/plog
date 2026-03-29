<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?= $title ?? 'Plog Blog' ?></title>
    <meta name="description" content="<?= $description ?? '' ?>">
    
    <!-- CSS -->
    <link rel="stylesheet" href="/themes/default/assets/css/style.css">
    
    <?php if (isset($assets)): ?>
    <?= $assets->renderCss() ?>
    <?php endif; ?>
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
            <div class="content">
                <?php if (isset($posts) && !empty($posts)): ?>
                    <!-- 文章列表 -->
                    <div class="post-list">
                        <?php foreach ($posts as $post): ?>
                        <article class="post-item">
                            <h2 class="post-title">
                                <a href="/post/<?= $post['id'] ?>"><?= htmlspecialchars($post['title']) ?></a>
                            </h2>
                            <div class="post-meta">
                                <span class="date"><?= date('Y-m-d', strtotime($post['created_at'])) ?></span>
                                <?php if (isset($post['category'])): ?>
                                <span class="category">
                                    <a href="/category/<?= $post['category']['id'] ?>"><?= htmlspecialchars($post['category']['name']) ?></a>
                                </span>
                                <?php endif; ?>
                            </div>
                            <?php if (isset($post['excerpt'])): ?>
                            <div class="post-excerpt"><?= htmlspecialchars($post['excerpt']) ?></div>
                            <?php endif; ?>
                            <a href="/post/<?= $post['id'] ?>" class="read-more">阅读全文 →</a>
                        </article>
                        <?php endforeach; ?>
                    </div>

                    <!-- 分页 -->
                    <?php if (isset($pagination)): ?>
                    <div class="pagination">
                        <?= $pagination ?>
                    </div>
                    <?php endif; ?>
                <?php else: ?>
                    <div class="no-posts">
                        <p>暂无文章</p>
                    </div>
                <?php endif; ?>
            </div>

            <!-- Sidebar -->
            <?php if ($show_sidebar ?? true): ?>
            <aside class="sidebar">
                <div class="widget">
                    <h3>分类</h3>
                    <ul>
                        <?php if (isset($categories)): ?>
                            <?php foreach ($categories as $category): ?>
                            <li>
                                <a href="/category/<?= $category['id'] ?>"><?= htmlspecialchars($category['name']) ?></a>
                                <span class="count">(<?= $category['post_count'] ?>)</span>
                            </li>
                            <?php endforeach; ?>
                        <?php endif; ?>
                    </ul>
                </div>

                <div class="widget">
                    <h3>标签</h3>
                    <div class="tags">
                        <?php if (isset($tags)): ?>
                            <?php foreach ($tags as $tag): ?>
                            <a href="/tag/<?= $tag['id'] ?>" class="tag"><?= htmlspecialchars($tag['name']) ?></a>
                            <?php endforeach; ?>
                        <?php endif; ?>
                    </div>
                </div>
            </aside>
            <?php endif; ?>
        </div>
    </main>

    <!-- Footer -->
    <footer class="footer">
        <div class="container">
            <p>&copy; <?= date('Y') ?> <?= $site_name ?? 'Plog' ?>. All rights reserved.</p>
            <p>Powered by <a href="https://plog.dev">Plog</a></p>
        </div>
    </footer>

    <!-- JavaScript -->
    <script src="/themes/default/assets/js/main.js"></script>
    <?php if (isset($assets)): ?>
    <?= $assets->renderJs() ?>
    <?php endif; ?>
</body>
</html>
