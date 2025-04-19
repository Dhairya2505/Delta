use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta {
    filename: String,
    file_id: String,
    is_private: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    id: String,
    name: String,
    files: Vec<FileMeta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    username: String,
    repos: Vec<Repo>,
}

pub async fn fetch_user_by_username(username: &str) -> Option<User> {
    dotenvy::dotenv().ok();
    let client = DynamoDbClient::new(Region::ApSouth1);

    let mut key = HashMap::new();
    key.insert("username".to_string(), to_attr_val(username));

    let input = GetItemInput {
        table_name: "delta".to_string(),
        key,
        ..Default::default()
    };

    let result = client.get_item(input).await.ok()?;

    if let Some(item) = result.item {
        let json = rusoto_to_json(&item);
        let user: User = serde_json::from_value(json).ok()?;
        Some(user)
    } else {
        None
    }
}

fn rusoto_to_json(item: &HashMap<String, AttributeValue>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in item.iter() {
        map.insert(k.clone(), attr_to_json(v));
    }
    serde_json::Value::Object(map)
}

fn to_attr_val(val: &str) -> AttributeValue {
    AttributeValue {
        s: Some(val.to_string()),
        ..Default::default()
    }
}

fn attr_to_json(val: &AttributeValue) -> serde_json::Value {
    if let Some(s) = &val.s {
        return serde_json::Value::String(s.clone());
    }
    if let Some(n) = &val.n {
        return serde_json::Value::Number(
            serde_json::Number::from_f64(n.parse::<f64>().unwrap()).unwrap(),
        );
    }
    if let Some(b) = val.bool {
        return serde_json::Value::Bool(b);
    }
    if let Some(l) = &val.l {
        return serde_json::Value::Array(l.iter().map(attr_to_json).collect());
    }
    if let Some(m) = &val.m {
        let map = m
            .iter()
            .map(|(k, v)| (k.clone(), attr_to_json(v)))
            .collect();
        return serde_json::Value::Object(map);
    }
    serde_json::Value::Null
}
