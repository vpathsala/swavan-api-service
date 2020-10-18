use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Mock {
    pub id: String,
    #[serde(rename(serialize = "secret"))]
    #[serde(rename(deserialize = "key"))]
    pub key: String,
}
