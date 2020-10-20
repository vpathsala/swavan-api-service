use std::env;
pub struct Configuration {
    pub api_server: String
}

impl Default for Configuration {
    fn default() -> Configuration {
        let server = match env::var("MOCK_API_URL") {
            Ok(val) => val,
            Err(_e) => "https://api.mocky.io/api/mock".to_owned(),
        };
        Configuration {
            api_server: server
        }
    }
}


pub fn host() -> String {
    format!("{}:{}",server(),port())
}

fn port() -> String {
    match env::var("MOCK_SERVER_PORT") {
        Ok(val) => val,
        Err(_e) => "8080".to_owned(),
    }
}

fn server() -> String {
    match env::var("MOCK_SERVER_URL") {
        Ok(val) => val,
        Err(_e) => "0.0.0.0".to_owned(),
    }
}
