use serde::{Serialize, Deserialize, Deserializer, de};
use std::collections::HashMap;
use std::str;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mock{
    pub charset: String,
    #[serde(deserialize_with = "content_ascii_to_string")]
    pub content: String,
    #[serde(rename(serialize = "content_type"))]
    #[serde(rename(deserialize = "contentType"))]
    pub content_type: String,
    #[serde(deserialize_with = "header_hash_to_list")]
    pub headers: Vec<super::common::Header>,
    #[serde(deserialize_with = "status_to_string")]
    pub status: String
}

pub fn status_to_string<'de, D>( deserializer: D, ) -> Result<String, D::Error>
where D: Deserializer<'de>, {
    let s: i16 = de::Deserialize::deserialize(deserializer)?;
    Ok(s.to_string())
}

pub fn content_ascii_to_string<'de, D>( deserializer: D, ) -> Result<String, D::Error>
where D: Deserializer<'de>, {
    let s = de::Deserialize::deserialize(deserializer)?;
    String::from_utf8(s).map_err(serde::de::Error::custom)
}

pub fn header_hash_to_list<'de, D>( deserializer: D, ) -> Result<Vec<super::common::Header>, D::Error>
where D: Deserializer<'de>, {
    let data: Option<HashMap<String, String>> = de::Deserialize::deserialize(deserializer)?;
    let mut headers:Vec<super::common::Header> = Vec::new();
            
    return match data {
        Some(rows) => {
            for (_key, _value) in rows.iter() {
                headers.push(super::common::Header{ key: _key.to_string(), value: _value.to_string() })
            }
            Ok(headers)
        },
        None => Ok(headers)
    }
}
