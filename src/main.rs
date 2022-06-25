mod handlers;
use actix_web::{App, HttpServer, web};
use mongodb::{bson::doc, options::ClientOptions, Client};
use dotenv;
use std::io::{Error, ErrorKind};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(error) = mongo_connect().await{
        return Err(Error::new(ErrorKind::Other, error.to_string()));
    }
    HttpServer::new(|| {
        App::new()
        .route("",web::get().to(handlers::warmup::hello))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn mongo_connect() -> mongodb::error::Result<()>  {
    if let Some(connection_string) = dotenv::var("MONGO_URI").ok() {
         let mut client_options =
        ClientOptions::parse(connection_string)
            .await?;
    client_options.app_name = Some("Blog Articles".to_string());
    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    return Ok(());
    }
    Ok(())
}

