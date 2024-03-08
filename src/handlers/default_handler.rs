use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola!")
}

#[post("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("I am up and running!")
}
