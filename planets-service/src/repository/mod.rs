use std::env;
extern crate dotenv;
use dotenv::dotenv;

//use bson::{from_bson, oid::ObjectId, to_bson, Bson, Document};
use mongodb::{
    Database, 
    Client, Collection, options::ClientOptions
};

//use crate::models::{Post, Author};
//use async_trait::async_trait;

#[derive(Clone)]
pub struct DB {
    pub client: Client,
    pub db_name: String,
    pub db: Database, 
    //pub collections: Collections<Post, Author>,
}

// #[derive(Clone)]
// pub struct Collections<Y, T>{
//     pub posts: Collection<Y>,
//     pub authors: Collection<T>,
// }

impl DB {

    pub async fn new<S>(db_name: S) -> Self//FieldResult<Db>
    where
        S: ToString,
    {

        dotenv().ok();
        let db_url = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };


        let db_name = db_name.to_string();


        // Parse a connection string into an options struct.
        let client_options = ClientOptions::parse(&db_url).expect("I Can't parse ClientOptions");

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).expect("I Can't get Client");
 
        // Get a handle to a database.
        let db: Database = client.database(&db_name);

        // let collections = Collections{ 
        //     posts: db.collection("posts"),
        //     authors: db.collection("authors"),
        // };

        DB { 
            client, 
            db_name, 
            db,
           // collections
        }
    }

    pub fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

}

// #[async_trait]
// impl PostDB for DB { }

mod post_coll;