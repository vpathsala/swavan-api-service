use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Header{
    pub key: String,
    pub value: String
}
