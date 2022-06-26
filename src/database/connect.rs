use mongodb::results::InsertOneResult;
use futures::StreamExt;
use crate::database::model::*;
use mongodb::{Collection, bson::{doc, extjson::de::Error, oid::ObjectId}, options::ClientOptions, Client, };
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


    pub async fn get_blog(&self, id: &String) -> Result<Vec<Blog>, Error> {
        let mut filter= None;
        if id != "" {
            let obj_id = ObjectId::parse_str(id)?;
            filter = Some(doc! {"_id": obj_id});
        }
         let mut cursor = self
                .col
                .find(filter, None)
                .await
                .ok()
                .expect("Error getting blog");
         let mut blogs: Vec<Blog> = Vec::new();
         while let Some(Ok(blog)) = cursor.next().await {
            blogs.push(blog);
         }
        Ok(blogs)

    }


    pub async fn post_blog(&self, new_blog: Blog) ->  Result<InsertOneResult, Error> {
        let new_doc = Blog {
            id: None,
            timeStamp: new_blog.timeStamp,
            title: new_blog.title,
            subject: new_blog.subject,
            preview: new_blog.preview,
            blogText: new_blog.blogText
        };
        return Ok(self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user"));
    }

}

