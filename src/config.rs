// config.rs
pub struct Config {
    pub db_connection_string: String,
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_connection_string: "mysql://leyu:123456@localhost:3306/bitcoin_explorer".to_string(),
            rpc_url: "http://127.0.0.1:8332".to_string(),
            rpc_user: "yourusername".to_string(),
            rpc_password: "yourpassword".to_string(),
        }
    }
}