# simple-auth-server

参考：
https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

## `Contents`
- [simple-auth-server](#simple-auth-server)
  - [`Contents`](#contents)
  - [环境](#%e7%8e%af%e5%a2%83)
  - [其他事项](#%e5%85%b6%e4%bb%96%e4%ba%8b%e9%a1%b9)
  - [访问示例](#%e8%ae%bf%e9%97%ae%e7%a4%ba%e4%be%8b)



## 环境
* rust 环境
* Postgres 环境，数据库
* Diesel 环境，管理数据库

环境安装参考：[Rust Actix-Web 验证 Auth Web 微服务](https://github.com/flyq/blogs/blob/master/Rust%20%E5%AD%A6%E4%B9%A0/token%20%E8%AE%A4%E8%AF%81/README.md)。

## 其他事项
注意新建 .env 文件并且设置：
```.env
DATABASE_URL=postgres://<database user>:<database user's password>@localhost/<database name>
SPARKPOST_API_KEY='<mail's KEY>'
SENDING_EMAIL_ADDRESS='<mail's address>'
```

然后运行：
```shell
cargo run
```
服务就启动了。

## 访问示例
```shell
$ curl --request POST   --url http://localhost:3000/api/invitation   --header 'content-type: application/json'   --data '{"email":"flyq951@gmail.com"}'

{
    "id": "67ff8c73-cd16-4bd7-8931-87aa1fc3d9fa",
    "email": "test@test.com",
    "expires_at": "2018-10-23T09:49:12.167510"
}

$ curl --request POST \
  --url http://localhost:3000/api/register/67ff8c73-cd16-4bd7-8931-87aa1fc3d9fa \
  --header 'content-type: application/json' \
  --data '{"password":"password"}'

BadRequest: Key (email)=(flyq951@gmail.com) already exists.




$ curl -i --request POST \
>   --url http://localhost:3000/api/auth \
>   --header 'content-type: application/json' \
>   --data '{"email": "flyq951@gmail.com","password":"password"}'
HTTP/1.1 200 OK
content-length: 0
set-cookie: auth=WAfszoYb9iP/uG8OCf5X05syHSrHOPDkegq4RbX7NV63EcZTt+/X8ZmnFZL2gOz1fMp+xTpwsgil; HttpOnly; Path=/; Domain=localhost; Max-Age=86400
date: Thu, 05 Mar 2020 09:29:01 GMT

```

```shell
[2020-03-05T09:27:09Z INFO  actix_web::middleware::logger] 127.0.0.1:47310 "POST /api/register/67ff8c73-cd16-4bd7-8931-87aa1fc3d9fa HTTP/1.1" 200 29 "-" "curl/7.58.0" 2.171168
```

