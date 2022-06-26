use crate::database::model::Blog;
use actix_web::{HttpResponse, Responder, web::{Data, Json}};
use crate::database::connect::MongoRepo;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    id: String,
}


pub async fn get_blog(db: Data<MongoRepo<Blog>>, info: Json<Info> ) -> impl Responder {

    let blog_details = db.get_blog(&info.id).await;
    match blog_details {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
