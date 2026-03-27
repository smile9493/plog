<?php

/**
 * Service: Util
 *
 * @package PLOG
 * 
 */

class Util
{

    /**
     * Check if the application is running in development environment
     */
    public static function isDevEnv()
    {
        return getenv('PLOG_ENV') === 'develop' || (defined('ENVIRONMENT') && ENVIRONMENT === 'develop');
    }
}
