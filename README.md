Bitcoin Explorer API
Overview
Bitcoin Explorer API is a backend service built with Rust and Actix Web that fetches blockchain data from a Bitcoin Core client and stores it in a MySQL database. It provides an HTTP API for front-end applications to retrieve the latest blockchain information.

Technology Stack
Language: Rust
Web Framework: Actix Web
Database: MySQL
Libraries: request for HTTP client requests, serde for JSON serialization, mysql Rust crate for database interactions

Setup
Prerequisites
Rust (install via rustup)
MySQL database
Local or remote Bitcoin Core client
Configuration
Clone the repository and navigate to the project directory:

bash
git clone [repository link]
cd [project folder]
Update the configuration in config.rs with the correct database connection string and Bitcoin Core RPC credentials:

rust
pub struct Config {
pub db_connection_string: String,
pub rpc_url: String,
pub rpc_user: String,
pub rpc_password: String,
}
Ensure the MySQL database is running and create the necessary tables:

sql
复制代码
CREATE TABLE Blocks (
id INT AUTO_INCREMENT PRIMARY KEY,
bestblockhash VARCHAR(255) NOT NULL,
blocks BIGINT,
chain VARCHAR(100),
...
);
Build and Run
Build the project:

arduino
复制代码
cargo build --release
Run the project:

arduino
复制代码
cargo run --release
API Endpoints
GET /blockchain-info: Retrieves the latest blockchain information.
