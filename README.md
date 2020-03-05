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
$ curl --request POST   --url http://localhost:3000/api/invitation   --header 'content-type: application/json'   --data '{"email":"flyqtest1@gmail.com"}'


# log:
{
    id: 6889d452-467e-45e2-a2cb-3ea0523e6e71,
    email: "flyqtest1@gmail.com",
    expires_at: 2020-03-06T10:28:19.546105,
}

```

```shell

$ curl --request POST \
>   --url http://localhost:3000/api/register/6889d452-467e-45e2-a2cb-3ea0523e6e71 \
>   --header 'content-type: application/json' \
>   --data '{"password":"flyqtestpw"}'

# return
{"email":"flyqtest1@gmail.com"}

# log
[2020-03-05T10:28:50Z INFO  actix_web::middleware::logger] 127.0.0.1:47558 "POST /api/register/6889d452-467e-45e2-a2cb-3ea0523e6e71 HTTP/1.1" 200 31 "-" "curl/7.58.0" 2.163482

```

如果使用改 token 再注册一次，则提示错误：
```shell
BadRequest: Key (email)=(flyqtest1@gmail.com) already exists.
```

得到cookie：
```shell
$ curl -i --request POST \
>   --url http://localhost:3000/api/auth \
>   --header 'content-type: application/json' \
>   --data '{"email": "flyqtest1@gmail.com","password":"flyqtestpw"}'

# return
HTTP/1.1 200 OK
content-length: 0
set-cookie: auth=WAfszoYb9iP/uG8OCf5X05syHSrHOPDkegq4RbX7NV63EcZTt+/X8ZmnFZL2gOz1fMp+xTpwsgil; HttpOnly; Path=/; Domain=localhost; Max-Age=86400
date: Thu, 05 Mar 2020 09:29:01 GMT

# log
[2020-03-05T09:27:09Z INFO  actix_web::middleware::logger] 127.0.0.1:47310 "POST /api/register/67ff8c73-cd16-4bd7-8931-87aa1fc3d9fa HTTP/1.1" 200 29 "-" "curl/7.58.0" 2.171168
```

