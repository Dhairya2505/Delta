use mongodb::{bson::doc, options::ClientOptions, Client};
use dotenv;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

#[tokio::main]
pub async fn check_username(username: &String) -> Result<bool, Box<dyn Error>> {

    let db_url = dotenv::var("db_url").unwrap();
    let client_options = ClientOptions::parse(db_url).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("delta");
    let collection = db.collection::<User>("user");

    let filter = doc! { "username": username };
    let user = collection.find_one(filter).await?;

    Ok(user.is_some())

}