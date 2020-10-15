use serde::{Serialize, Deserialize, Deserializer, de};
use std::collections::HashMap;
use std::str;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MockPayloadModal{
    pub charset: String,
    #[serde(deserialize_with = "content_ascii_to_string")]
    pub content: String,
    pub content_type: String,
    #[serde(deserialize_with = "header_hash_to_list")]
    pub headers: Vec<MockHeaderModal>,
    pub status: i16
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MockHeaderModal{
    pub key: String,
    pub value: String
}

pub fn content_ascii_to_string<'de, D>( deserializer: D, ) -> Result<String, D::Error>
where D: Deserializer<'de>, {
    let s = de::Deserialize::deserialize(deserializer)?;
    String::from_utf8(s).map_err(serde::de::Error::custom)
}

pub fn header_hash_to_list<'de, D>( deserializer: D, ) -> Result<Vec<MockHeaderModal>, D::Error>
where D: Deserializer<'de>, {
    let data: Option<HashMap<String, String>> = de::Deserialize::deserialize(deserializer)?;
    let mut headers:Vec<MockHeaderModal> = Vec::new();
            
    return match data {
        Some(rows) => {
            for (_key, _value) in rows.iter() {
                headers.push(MockHeaderModal{ key: _key.to_string(), value: _value.to_string() })
            }
            Ok(headers)
        },
        None => Ok(headers)
    }

    //String::from_utf8(s).map_err(serde::de::Error::custom)
}