# Rust Web开发一
[TOC]

<https://actix-web.budshome.com/>

## 前言
别人写书可能到了多线程就结束了，我一定会写一写和Web相关的东西。因为我一直是做和Web相关的工作。

说点有趣的东西，我最近一段时间面试程序员，他们普遍都会说一句：我不想从事CRUD[^1]相关的工作。我其实很无语，我主要从事的都是互联网的业务相关的工作，说实话完全不用CRUD的事情几乎没有。

当然现在有些做Ai算法的工作，他们主要是和数学公式相关，各种矩阵变换相关，他们看似和CRUD不打交道，那是因为他们把这些打交道的事情交给了工程相关的人。

闲话少叙，直转正题。我为啥要学习Rust？主要是因为一次偶然的机会，看到一个网站————<https://www.techempower.com/benchmarks/>。它在Round 18的时候评测个各种web架构，在这次评测的时候，不论是物理机还是云主机，第一名都是一个Rust开发的框架————Actix。这个评测完全是基于响应速度，所以我们大多数熟悉的框架的排名都很低。不过这两年的排名，第一都是C++ 的drogon。

今年actix的新一代产品ntex也超过了actix。不过这都不重要，开发重要的是社区和文档，这个actix要好很多。

下面我就基于actix-web给大家聊一下怎么使用Rust开发web。

## actix-web Hello World

```shell
$ cargo new actix_helloworld
$ cd actix_helloworld
```

```toml
[dependencies]
actix-web = "3"
```

```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
编译的时候要多等一会，因为要下载212个crate。

打开浏览器，如果有参数就 Hello `参数`，如果没有参数就 Hello World!

为了体会Rust的`快感`,你可以试试使用wrk。<https://github.com/wg/wrk>

```shell
$ wrk -t12 -c400 -d30s http://127.0.0.1:8080
```

## 具备请求路由器
actix 具备 URL 路由系统，可以匹配 URL 并调用各个 handler。把上面的程序进行修改：
```shell
$ cargo actix_hellorouter
```

```rust
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

// #[get("/{name}")]
// async fn index(web::Path(mut name): web::Path<String>) -> impl Responder {
//     if name.len()==0{
//         name=" World!".to_string();
//     }
//     format!("Hello {}!", name)
// }
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}

async fn greet(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("带路由的actix-web::0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

```

也可以稍微改写一下：
```rust
async fn greet(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
```

## 加强路由

上面虽然可以运行，但是感觉总是哪里别扭，不是应该每个函数都可以使用宏路由吗？下面我们进行修改：
```shell
$ cargo new actix_router
```
代码如下：

```rust
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}
#[get("/{name}")]
async fn greet(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("带路由的actix-web::0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

## 解释
[^1]: CRUD:crud是指在做计算处理时的增加(Create)、检索(Retrieve)、更新(Update)和删除(Delete)几个单词的首字母简写。crud主要被用在描述软件系统中数据库或者持久层的基本操作功能。