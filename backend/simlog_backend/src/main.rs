use crate::server::Server;

mod handlers;
mod models;
mod router;
mod server;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    // Load server and database addresses
    let server_address =
        std::env::var("BACKEND_ADDRESS").expect(".env error: no value set for BACKEND_ADDRESS");
    let db_url = std::env::var("DATABASE_URL").expect(".env error: no value set for DATABASE_URL");

    // Attempt to create server
    match Server::new(server_address, db_url).await {
        Ok(server) => {
            // Attempt to start server
            match server.start().await {
                Ok(_) => {
                    // Server is running
                }
                Err(e) => {
                    println!("{e}")
                }
            }
        }
        Err(e) => {
            println!("{e}")
        }
    }
}
