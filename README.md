# simple-auth-server

参考：
https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

## `Contents`
- [simple-auth-server](#simple-auth-server)
  - [`Contents`](#contents)
  - [环境](#%e7%8e%af%e5%a2%83)
  - [步骤](#%e6%ad%a5%e9%aa%a4)
    - [1. 按照环境把环境配置好，在postgres里面生成数据库](#1-%e6%8c%89%e7%85%a7%e7%8e%af%e5%a2%83%e6%8a%8a%e7%8e%af%e5%a2%83%e9%85%8d%e7%bd%ae%e5%a5%bd%e5%9c%a8postgres%e9%87%8c%e9%9d%a2%e7%94%9f%e6%88%90%e6%95%b0%e6%8d%ae%e5%ba%93)
    - [2. 拉取代码](#2-%e6%8b%89%e5%8f%96%e4%bb%a3%e7%a0%81)
    - [3. 修改配置文件 .env](#3-%e4%bf%ae%e6%94%b9%e9%85%8d%e7%bd%ae%e6%96%87%e4%bb%b6-env)
    - [4. 新建数据库 simple-auth-server](#4-%e6%96%b0%e5%bb%ba%e6%95%b0%e6%8d%ae%e5%ba%93-simple-auth-server)
    - [5. 然后运行：](#5-%e7%84%b6%e5%90%8e%e8%bf%90%e8%a1%8c)
  - [访问示例](#%e8%ae%bf%e9%97%ae%e7%a4%ba%e4%be%8b)



## 环境
* rust 环境
* Postgres 环境，数据库
* Diesel 环境，管理数据库

环境安装参考：[Rust Actix-Web 验证 Auth Web 微服务](https://github.com/flyq/blogs/blob/master/Rust%20%E5%AD%A6%E4%B9%A0/token%20%E8%AE%A4%E8%AF%81/README.md)。

## 步骤
### 1. 按照[环境](#%e7%8e%af%e5%a2%83)把环境配置好，在postgres里面生成数据库

### 2. 拉取代码
```shell
git clone https://github.com/flyq/simple-auth-server.git
```
### 3. 修改配置文件 .env
注意新建 .env 文件并且设置：
```.env
DATABASE_URL=postgres://<database user>:<database user's password>@localhost/<database name>
SPARKPOST_API_KEY='<mail's KEY>'
SENDING_EMAIL_ADDRESS='<mail's address>'
```
参考：
```shell
DATABASE_URL=postgres://ubuntu:123456@localhost/simple-auth-server
SPARKPOST_API_KEY='111'
SENDING_EMAIL_ADDRESS='11111@163.com'
```
### 4. 新建数据库 simple-auth-server
```shell
$ diesel setup
Creating database: simple-auth-server
Running migration 2020-03-04-091801_users
Running migration 2020-03-04-091813_invitations
```
创建成功。

### 5. 然后运行：
```shell
cargo run
```
服务就启动了。

## 访问示例
```shell
$ curl --request POST   --url http://localhost:3000/api/invitation   --header 'content-type: application/json'   --data '{"email":"flyqtest1@gmail.com"}'


# log:
{
    id: 6889d452-467e-45e2-a2cb-3ea0523e6e71,
    email: "flyqtest1@gmail.com",
    expires_at: 2020-03-06T10:28:19.546105,
}

```

```shell

$ curl --request POST  --url http://localhost:3000/api/register/6889d452-467e-45e2-a2cb-3ea0523e6e71  --header 'content-type: application/json'  --data '{"password":"flyqtestpw"}'

# return
{"email":"flyqtest1@gmail.com"}

# log
[2020-03-05T10:28:50Z INFO  actix_web::middleware::logger] 127.0.0.1:47558 "POST /api/register/6889d452-467e-45e2-a2cb-3ea0523e6e71 HTTP/1.1" 200 31 "-" "curl/7.58.0" 2.163482

```

如果使用该 token 再注册一次，则提示错误：
```shell
BadRequest: Key (email)=(flyqtest1@gmail.com) already exists.
```

得到cookie：
```shell
$ curl -i --request POST --url http://localhost:3000/api/auth  --header 'content-type: application/json'  --data '{"email": "flyqtest1@gmail.com","password":"flyqtestpw"}'

# return
HTTP/1.1 200 OK
content-length: 0
set-cookie: auth=WAfszoYb9iP/uG8OCf5X05syHSrHOPDkegq4RbX7NV63EcZTt+/X8ZmnFZL2gOz1fMp+xTpwsgil; HttpOnly; Path=/; Domain=localhost; Max-Age=86400
date: Thu, 05 Mar 2020 09:29:01 GMT

# log
[2020-03-05T09:27:09Z INFO  actix_web::middleware::logger] 127.0.0.1:47310 "POST /api/register/67ff8c73-cd16-4bd7-8931-87aa1fc3d9fa HTTP/1.1" 200 29 "-" "curl/7.58.0" 2.171168



curl -i --request GET  --url http://localhost:3000/auth  --cookie auth=WAfszoYb9iP/uG8OCf5X05syHSrHOPDkegq4RbX7NV63EcZTt+/X8ZmnFZL2gOz1fMp+xTpwsgil


#log
HTTP/1.1 200 OK
content-length: 31
content-type: application/json
date: Wed, 08 Apr 2020 07:38:05 GMT

{"email":"flyqtest1@gmail.com"}
```

