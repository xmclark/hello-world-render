use actix_web::{web, App, HttpServer, Responder, HttpResponse};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/healthz", web::get().to(index2))
    })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .unwrap();
}