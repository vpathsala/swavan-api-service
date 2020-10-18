use std::env;
pub struct Configuration {
    pub api_server: String
}

impl Default for Configuration {
    fn default() -> Configuration {
        let server = match env::var("MOCK_SERVER_URL") {
            Ok(val) => val,
            Err(_e) => "https://api.mocky.io/api/mock".to_owned(),
        };
        Configuration {
            api_server: server
        }
    }
}
