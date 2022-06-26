// Modules
mod database;
mod handlers;


//Dependencies
use actix_web::web::Data;
use actix_web::{App, HttpServer, web};
use std::io::{Error, ErrorKind};
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match database::connect::MongoRepo::init().await {
        Ok(Some(db)) => {
            println!( "here");
            let db_data = Data::new(db);
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            HttpServer::new(move || {
                App::new()
                .app_data(db_data.clone())
                .route("/",web::post().to(handlers::post_blog::post_blog))
                .route("/getblogpost",web::post().to(handlers::get_blog::get_blog))

            })
            .bind(format!("{}:{}", host, port))?
            .run()
            .await
        },
        Ok(None) => return Err(Error::new(ErrorKind::Other, "Could Not Connect to MongoDB")),
        Err(error) => return Err(Error::new(ErrorKind::Other, error.to_string())),
    }
}




