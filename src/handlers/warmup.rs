use actix_web::{HttpResponse, Responder};
use crate::database;
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
