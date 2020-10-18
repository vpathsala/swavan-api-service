use serde::{Serialize, Deserialize,};
use std::str;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    #[serde(rename(deserialize = "secret"))]
    pub key: Option<String>,
    pub link: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Mock<T>{
    #[serde(default = "default_charset")]
    pub charset: String,
    pub content: String,
    pub content_type: String,
    pub headers: T,
    pub status: String,
    #[serde(default = "default_expire")]
    pub expiration: String,
    #[serde(rename(serialize = "secret"))]
    pub key: String,
    pub id: Option<String>
}

pub fn default_charset() -> String {
    format!("UTF-8")
}

pub fn default_expire() -> String {
    format!("never")
}

pub fn mock_to_post(payload: &Mock<Vec<super::common::Header>>) ->
Mock<HashMap<String, String>> {
    let mut hash_map:HashMap<String, String> = HashMap::new();
    for row in payload.headers.iter() {
      hash_map.insert(row.key.to_string(), row.value.to_string());
    }
    Mock{
        headers: hash_map,
        charset: payload.charset.to_owned(),
        content: payload.content.to_owned(),
        content_type: payload.content_type.to_owned(),
        status: payload.status.to_owned(),
        expiration: payload.expiration.to_owned(),
        key: payload.key.to_owned(),
        id: payload.id.to_owned()
    }
}