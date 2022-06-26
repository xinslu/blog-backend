// Modules
mod database;
mod handlers;


//Dependencies
use actix_web::web::Data;
use actix_web::{App, HttpServer, web};
use std::io::{Error, ErrorKind};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match database::connect::MongoRepo::init().await {
        Ok(Some(db)) => {
            println!( "here");
            let db_data = Data::new(db);
            HttpServer::new(move || {
                App::new()
                .app_data(db_data.clone())
                .route("/getblogpost",web::post().to(handlers::get_blog::get_blog))

            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        },
        Ok(None) => return Err(Error::new(ErrorKind::Other, "Could Not Connect to MongoDB")),
        Err(error) => return Err(Error::new(ErrorKind::Other, error.to_string())),
    }
}




