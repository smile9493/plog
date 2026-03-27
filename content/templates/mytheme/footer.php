<?php
if(!defined('PLOG_ROOT')) {exit('error!');}
?>
    </main>
    
    <footer class="site-footer">
        <div class="footer-content">
            <p>&copy; <?= date('Y') ?> <a href="<?= BLOG_URL ?>"><?= $blogname ?></a>. All rights reserved.</p>
            <p>Powered by <a href="https://www.plog.net" target="_blank">Plog</a> <?= Option::PLOG_VERSION ?></p>
            <?php if($icp): ?>
            <p><?= $icp ?></p>
            <?php endif; ?>
            <?php if($footer_info): ?>
            <p><?= $footer_info ?></p>
            <?php endif; ?>
        </div>
    </footer>
    
    <?php doAction('index_footer') ?>
</body>
</html>
