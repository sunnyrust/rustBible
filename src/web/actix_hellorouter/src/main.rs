use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}

async fn greet(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("带路由的actix-web::0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .service(index).route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
