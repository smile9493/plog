<?php
/*
Plugin Name: 模板设置
Version: 1.0.0
Plugin URL: https://www.plog.net
Description: 用于模板设置的插件，系统内置插件不可禁用
Author: Plog Team
Author URL: https://www.plog.net
*/

defined('PLOG_ROOT') || exit('access denied!');

if (!function_exists('_g')) {
    function _g($key, $default = '')
    {
        return TplOptions::get($key, $default);
    }
}

if (!function_exists('_em')) {
    function _em($key, $default = '')
    {
        return TplOptions::get($key, $default);
    }
}

class TplOptions
{
    private static $options = null;
    
    public static function get($key, $default = '')
    {
        if (self::$options === null) {
            self::loadOptions();
        }
        
        return isset(self::$options[$key]) ? self::$options[$key] : $default;
    }
    
    private static function loadOptions()
    {
        self::$options = [];
        $template = Template::getCurrentTemplate();
        if (empty($template)) {
            return;
        }
        
        $db = Database::getInstance();
        $sql = "SELECT name, data FROM " . DB_PREFIX . "tpl_options_data WHERE template = '" . $db->escape_string($template) . "'";
        $result = $db->query($sql);
        while ($row = $db->fetch_array($result)) {
            $data = $row['data'];
            if (is_string($data) && (substr($data, 0, 2) === 'a:' || substr($data, 0, 2) === 's:' || substr($data, 0, 2) === 'i:')) {
                $unserialized = @unserialize($data);
                if ($unserialized !== false) {
                    $data = $unserialized;
                }
            }
            self::$options[$row['name']] = $data;
        }
    }
    
    public static function updateOption($name, $data)
    {
        $template = Template::getCurrentTemplate();
        if (empty($template)) {
            return false;
        }
        
        $db = Database::getInstance();
        $serializedData = is_string($data) ? $data : serialize($data);
        $sql = "INSERT INTO " . DB_PREFIX . "tpl_options_data (template, name, data) VALUES ('" . $db->escape_string($template) . "', '" . $db->escape_string($name) . "', '" . $db->escape_string($serializedData) . "') ON DUPLICATE KEY UPDATE data = '" . $db->escape_string($serializedData) . "'";
        return $db->query($sql);
    }
}
