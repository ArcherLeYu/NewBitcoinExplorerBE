// config.rs
#[derive(Clone, Debug)]  // 为 Config 结构体派生 Clone 和 Debug trait
pub struct Config {
    pub db_connection_string: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_connection_string: "mysql://leyu:123456@db:3306/bitcoin_explorer".to_string(),
        }
    }
}

// impl Config {
//     pub fn new() -> Self {
//         Config {
//             db_connection_string: "mysql://leyu:123456@localhost:3306/bitcoin_explorer".to_string(),
//             //rpc_url: "http://127.0.0.1:8332".to_string(),
//             //rpc_user: "yourusername".to_string(),
//             //rpc_password: "yourpassword".to_string(),
//         }
//     }
// }

