#!/bin/bash

cargo build -r actix

#停止服务
docker stop actix-admin


#删除容器
docker rm -f actix-admin

#删除镜像
docker rmi -f actix-admin:v1

#删除none镜像
docker rmi -f $(docker images | grep "none" | awk '{print $3}')

#构建服务
docker build -t actix-admin:v1 -f Dockerfile .

#启动服务
docker run -itd --net=host --name=actix-admin actix-admin:v1
