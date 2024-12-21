// https://mp.weixin.qq.com/s/BKa6zSDXmShsqePADu-0Lw  Rust新手福音：常用Web框架大揭秘
#[cfg(test)]
mod tests {
    use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    // #[actix_web::main]
    #[tokio::test]
    async fn actix_web_server() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }

}
