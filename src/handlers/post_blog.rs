use crate::database::model::Blog;
use actix_web::{HttpResponse, Responder, web::{Data, Json}};
use crate::database::connect::MongoRepo;




pub async fn post_blog(db: Data<MongoRepo<Blog>>, body: Json<Blog> ) -> impl Responder {
    if body.timeStamp == "" || body.subject == "" || body.blogText == "" || body.title  == "" || body.preview == "" {
        return HttpResponse::InternalServerError().body("ERROR: None of the Fields can be blank");
    }
    let data = Blog {
            id: None,
            timeStamp: body.timeStamp.clone(),
            title: body.title.clone(),
            subject: body.subject.clone(),
            preview: body.preview.clone(),
            blogText: body.blogText.clone()
        };
    let blog_detail = db.post_blog(data).await;
    match blog_detail {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
