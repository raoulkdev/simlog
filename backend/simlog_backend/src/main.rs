use crate::server::Server;

mod router;
mod server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let server_address =
        std::env::var("BACKEND_ADDRESS").expect(".env error: no value set for BACKEND_ADDRESS");
    let db_url = std::env::var("DATABASE_URL").expect(".env error: no value set for DATABASE_URL");

    match Server::new(server_address, db_url).await {
        Ok(server) => {
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
