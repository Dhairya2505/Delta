use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileMeta {
    filename: String,
    file_id: String,
    is_private: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repo {
    id: String,
    name: String,
    files: Vec<FileMeta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    repos: Vec<Repo>,
}

fn to_attr_val(val: &str) -> AttributeValue {
    AttributeValue {
        s: Some(val.to_string()),
        ..Default::default()
    }
}

fn to_bool_attr_val(val: bool) -> AttributeValue {
    AttributeValue {
        bool: Some(val),
        ..Default::default()
    }
}

#[tokio::main]
pub async fn aws_fn(
    username: &String,
    repo_id: String,
    path_list: &Vec<(String, bool)>, // (path_string, is_private)
    repo_name: &String,
) {
    let table_name = "delta";
    dotenvy::dotenv().ok();
    let client = DynamoDbClient::new(Region::ApSouth1);

    let mut key = HashMap::new();
    key.insert("username".to_string(), to_attr_val(username));

    let get_input = GetItemInput {
        table_name: table_name.to_string(),
        key: key.clone(),
        ..Default::default()
    };

    let result = client.get_item(get_input).await.unwrap();

    let mut user: User = if let Some(item) = result.item {
        let json = rusoto_to_json(&item);
        serde_json::from_value(json).unwrap()
    } else {
        User {
            username: username.clone(),
            repos: vec![],
        }
    };

    let mut repo_opt = user.repos.iter_mut().find(|r| r.id == repo_id);
    if repo_opt.is_none() {
        user.repos.push(Repo {
            id: repo_id.clone(),
            name: repo_name.clone(),
            files: vec![],
        });
        repo_opt = user.repos.iter_mut().find(|r| r.id == repo_id);
    }

    let mut final_files = vec![];

    if let Some(repo) = repo_opt {
        for (path_string, is_private) in path_list {
            let new_file = FileMeta {
                filename: path_string.clone(),
                file_id: format!("{}{}", repo_id, &path_string[1..]),
                is_private: *is_private,
            };

            let mut found = false;
            for file in repo.files.iter_mut() {
                if file.filename == new_file.filename {
                    file.is_private = new_file.is_private;
                    found = true;
                    break;
                }
            }

            if !found {
                repo.files.push(new_file);
            }
        }

        // Clone files now that mutable borrow is done
        final_files = repo.files.clone();
    }

    // Put updated user
    let user_json = serde_json::to_value(&user).unwrap();
    let mut item = HashMap::new();

    if let serde_json::Value::Object(map) = user_json {
        for (key, value) in map {
            item.insert(key, json_to_attr(value));
        }
    }

    let put_input = PutItemInput {
        table_name: table_name.to_string(),
        item,
        ..Default::default()
    };

    client.put_item(put_input).await.unwrap();

}

fn json_to_attr(value: serde_json::Value) -> AttributeValue {
    match value {
        serde_json::Value::String(s) => to_attr_val(&s),
        serde_json::Value::Bool(b) => to_bool_attr_val(b),
        serde_json::Value::Array(arr) => AttributeValue {
            l: Some(arr.into_iter().map(json_to_attr).collect()),
            ..Default::default()
        },
        serde_json::Value::Object(obj) => {
            let map = obj.into_iter().map(|(k, v)| (k, json_to_attr(v))).collect();
            AttributeValue {
                m: Some(map),
                ..Default::default()
            }
        }
        serde_json::Value::Number(num) => AttributeValue {
            n: Some(num.to_string()),
            ..Default::default()
        },
        _ => AttributeValue::default(),
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

fn rusoto_to_json(item: &HashMap<String, AttributeValue>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in item.iter() {
        map.insert(k.clone(), attr_to_json(v));
    }
    serde_json::Value::Object(map)
}
