use crate::database::model::*;
use mongodb::{Collection, bson::doc, options::ClientOptions, Client};
use dotenv;
pub struct MongoRepo<Blog> {
    col: Collection<Blog>,
}
impl MongoRepo<Blog> {
    pub async fn init() -> Result<Option<MongoRepo<Blog>>, mongodb::error::Error>  {
        if let Some(connection_string) = dotenv::var("MONGO_URI").ok() {
            let mut client_options = ClientOptions::parse(connection_string).await?;
            client_options.app_name = Some("Blog Articles".to_string());
            let client = Client::with_options(client_options)?;
            client.database("admin").run_command(doc! {"ping": 1}, None).await?;
            println!("Connected successfully.");
            let db = client.database("myFirstDatabase");
            let col: Collection<Blog> = db.collection("blogs");
            return Ok(Some(MongoRepo{col}));
        }
        Ok(None)
    }


}

