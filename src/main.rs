use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

mod config;
mod db;

#[get("/")]
async fn greeting_message() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust for backend engineering")
}

#[get("/test")]
async fn testing_message() -> impl Responder {
    HttpResponse::Ok().body("Welcome to nazi")
}

#[post("/message")]
async fn message(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/user")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("testr")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greeting_message)
            .service(testing_message)
    })
    .bind("localhost:2121")?
    .run()
    .await
}
