FROM php:8.0-apache

LABEL maintainer="Plog Team <team@plog.dev>"
LABEL description="Plog CMS Monorepo Application"

# 设置时区
ENV TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# 使用阿里云镜像源
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list \
    && sed -i 's/security.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    git \
    unzip \
    zip \
    curl \
    libzip-dev \
    libpng-dev \
    libjpeg-dev \
    libfreetype6-dev \
    libonig-dev \
    libxml2-dev \
    && rm -rf /var/lib/apt/lists/*

# 安装 PHP 扩展
RUN docker-php-ext-configure gd --with-freetype --with-jpeg \
    && docker-php-ext-install -j$(nproc) \
    pdo \
    pdo_mysql \
    mysqli \
    zip \
    gd \
    mbstring \
    xml \
    bcmath

# 配置 Apache 允许 rewrite
RUN sed -i 's/AllowOverride None/AllowOverride All/' /etc/apache2/apache2.conf

# 启用 Apache mod_rewrite
RUN a2enmod rewrite

# 设置 Apache 默认字符集为 UTF-8
RUN echo 'AddDefaultCharset UTF-8' >> /etc/apache2/apache2.conf \
    && echo 'LoadModule rewrite_module modules/mod_rewrite.so' >> /etc/apache2/apache2.conf

# 安装 Composer
COPY --from=composer:2.0 /usr/bin/composer /usr/bin/composer

# 设置工作目录
WORKDIR /var/www/html

# 复制应用代码
COPY . .

# 安装依赖
RUN composer install --no-dev --optimize-autoloader

# 设置权限
RUN chown -R www-data:www-data /var/www/html \
    && chmod -R 755 /var/www/html

# 复制 Vue3 admin-web 构建文件
COPY apps/admin-web/dist /var/www/html/admin-web

# 配置 Apache 别名 - 将 /admin 指向 admin-web，并配置 API 路由
RUN echo 'Alias /admin /var/www/html/admin-web' >> /etc/apache2/sites-available/000-default.conf \
    && echo '<Directory /var/www/html/admin-web>' >> /etc/apache2/sites-available/000-default.conf \
    && echo '    Require all granted' >> /etc/apache2/sites-available/000-default.conf \
    && echo '    Options -Indexes' >> /etc/apache2/sites-available/000-default.conf \
    && echo '</Directory>' >> /etc/apache2/sites-available/000-default.conf \
    && echo '' >> /etc/apache2/sites-available/000-default.conf \
    && echo '# API routing - use RewriteRule' >> /etc/apache2/sites-available/000-default.conf \
    && echo 'RewriteEngine On' >> /etc/apache2/sites-available/000-default.conf \
    && echo 'RewriteCond %{REQUEST_URI} ^/api/' >> /etc/apache2/sites-available/000-default.conf \
    && echo 'RewriteRule ^api/(.*)$ /api.php [QSA,L]' >> /etc/apache2/sites-available/000-default.conf

# 暴露端口
EXPOSE 80

# 启动 Apache
CMD ["apache2-foreground"]
