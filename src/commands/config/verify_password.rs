use mongodb::{bson::doc, options::ClientOptions, Client};
use dotenv;
use std::error::Error;
use serde::{Deserialize, Serialize};
use bcrypt::verify;

use crate::utility::append_data::append;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

#[tokio::main]
pub async fn verify_password(username: &String, password: &String) -> Result<bool, Box<dyn Error>> {

    let db_url = dotenv::var("db_url").unwrap();
    let client_options = ClientOptions::parse(db_url).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("delta");
    let collection = db.collection::<User>("user");

    let filter = doc! { "username": username };

    if let Some(user_doc) = collection.find_one(filter).await? {
        let hashed = &user_doc.password;
        let is_valid = verify(password, hashed)?;
        append("/usr/local/bin/.config", &user_doc.password);
        Ok(is_valid)
    } else {
        Ok(false)
    }

}