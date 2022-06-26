use crate::database::model::Blog;
use actix_web::{HttpResponse, Responder, web::{Data, Json}};
use crate::database::connect::MongoRepo;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Info {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogReponse {
    success: bool,
    results: Vec<Blog>
}


pub async fn get_blog(db: Data<MongoRepo<Blog>>, info: Json<Info> ) -> impl Responder {
    let blog_details = db.get_blog(&info.id).await;
    match blog_details {
        Ok(blog) => {
            let result = BlogReponse {success: true, results: blog};
            HttpResponse::Ok().json(result)},
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
