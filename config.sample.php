<?php
header("location: ./install.php");
exit;
//MySQL database host
const DB_HOST = 'localhost';
//MySQL database username
const DB_USER = 'root';
//MySQL database user password
const DB_PASSWD = '';
//Database name
const DB_NAME = 'plog';
//Database table prefix
const DB_PREFIX = 'plog_';
//Auth key
const AUTH_KEY = 'plog-key';
//Cookie name
const AUTH_COOKIE_NAME = 'plog-cookie';