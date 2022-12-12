#!/bin/bash

cargo build -r

#停止服务
docker stop rust-admin


#删除容器
docker rm -f rust-admin

#删除镜像
docker rmi -f rust-admin:v1

#删除none镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

#构建服务
docker build -t rust-admin:v1 -f Dockerfile .

#启动服务
docker run -itd --net=host --name=rust-admin rust-admin:v1
