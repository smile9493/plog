#!/bin/bash
sudo sed -i "s|http://archive.ubuntu.com/ubuntu/|http://mirrors.aliyun.com/ubuntu/|g" /etc/apt/sources.list.d/ubuntu.sources
sudo sed -i "s|http://security.ubuntu.com/ubuntu/|http://mirrors.aliyun.com/ubuntu/|g" /etc/apt/sources.list.d/ubuntu.sources
grep URIs /etc/apt/sources.list.d/ubuntu.sources
