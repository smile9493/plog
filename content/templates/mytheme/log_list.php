<?php
if(!defined('PLOG_ROOT')) {exit('error!');}
require_once View::getView('header');
?>

<!-- Hero Section -->
<section class="hero-section">
    <h1 class="hero-title">发现技术价值 创造数字未来</h1>
    <p class="hero-subtitle">中国开发者首选的技术博客平台，每日精选优质内容</p>
</section>

<!-- 功能卡片区域 -->
<section class="features-section">
    <div class="feature-card">
        <i class="fas fa-rocket"></i>
        <h3>前沿技术</h3>
        <p>探索最新技术趋势和前沿开发技术</p>
        <span class="tag hot">热门</span>
    </div>
    
    <div class="feature-card">
        <i class="fas fa-users"></i>
        <h3>社区互动</h3>
        <p>与开发者交流分享经验心得</p>
        <span class="tag active">活跃</span>
    </div>
    
    <div class="feature-card">
        <i class="fas fa-graduation-cap"></i>
        <h3>学习路径</h3>
        <p>系统化的学习成长路线图</p>
        <span class="tag new">新手</span>
    </div>
</section>

<!-- 搜索栏 -->
<div class="search-section">
    <div class="search-box">
        <input type="text" placeholder="搜索技术文章、教程、资源..." class="search-input">
        <button class="search-btn"><i class="fas fa-search"></i></button>
    </div>
    <div class="hot-tags">
        <span class="tag-label">热门：</span>
        <a href="#" class="tag">前端开发</a>
        <a href="#" class="tag">人工智能</a>
        <a href="#" class="tag">Java</a>
        <a href="#" class="tag">Python</a>
        <a href="#" class="tag">数据库</a>
        <a href="#" class="tag">DevOps</a>
    </div>
</div>

<!-- 文章列表 -->
<div class="posts-section">
    <div class="section-header">
        <h2>最新文章</h2>
        <div class="stats">
            <span class="stat-item"><strong>25</strong> 篇技术文章</span>
            <span class="stat-item"><strong>14,901</strong> 总阅读量</span>
            <span class="stat-item"><strong>19</strong> 活跃作者</span>
        </div>
    </div>
    
    <?php doAction('index_loglist_top') ?>
    
    <div class="posts-grid">
        <?php if($logs): ?>
            <?php foreach($logs as $value): ?>
            <article class="post-card">
                <?php if($value['log_cover']): ?>
                <div class="post-cover">
                    <img src="<?= $value['log_cover'] ?>" alt="<?= $value['log_title'] ?>">
                </div>
                <?php endif; ?>
                
                <div class="post-info">
                    <h3 class="post-title">
                        <a href="<?= $value['log_url'] ?>"><?= $value['log_title'] ?></a>
                    </h3>
                    
                    <div class="post-meta">
                        <span class="date"><i class="far fa-clock"></i> <?= date('Y-m-d', $value['date']) ?></span>
                        <span class="views"><i class="far fa-eye"></i> <?= $value['views'] ?></span>
                        <span class="comments"><i class="far fa-comment"></i> <?= $value['comnum'] ?></span>
                    </div>
                    
                    <div class="post-excerpt">
                        <?= mb_substr(strip_tags($value['log_description']), 0, 120) ?>...
                    </div>
                    
                    <div class="post-tags">
                        <?= blog_tag($value['logid']) ?>
                    </div>
                </div>
            </article>
            <?php endforeach; ?>
        <?php else: ?>
            <div class="no-posts">
                <p>暂无文章</p>
            </div>
        <?php endif; ?>
    </div>
    
    <div class="load-more">
        <button class="load-more-btn">
            <i class="fas fa-arrow-down"></i>
            加载更多
        </button>
    </div>
</div>

<?php
require_once View::getView('footer');
?>
