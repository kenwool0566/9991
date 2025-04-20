use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hi")
}

#[get("/favicon.ico")]
pub async fn favicon() -> impl Responder {
    HttpResponse::Ok().content_type("image/x-icon").body("Hi")
}
