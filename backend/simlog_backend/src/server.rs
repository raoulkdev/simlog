use std::{sync::Arc};

use colored::Colorize;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::router;

pub struct Server {
    address: String,
    db: Arc<Pool<Postgres>>,
}

impl Server {
    pub async fn new(address: String, db_url: String) -> Result<Self, String> {
        Ok(
            Self {
                address,
                db: Self::setup_db(&db_url).await?
            }
        )
    }

    pub async fn start(&self) -> Result<(), String> {
        match TcpListener::bind(&self.address).await {
            Ok(listener) => {
                println!("{}{}", "Sucessfully started server at: ".bright_green(), &self.address);
                Ok(
                    axum::serve(listener, router::router(self.db.clone()))
                        .await
                        .map_err(|e| format!("{}{}, Error: {}", "Error staring server at: ".bright_red(), &self.address, e))?
                )
        
            }
            Err(e) => {
                Err(format!("{}{}, Error: {}", "Error starting server at: ".bright_red(), &self.address, e))
            }
        }
    }

    async fn setup_db(db_url: &str) -> Result<Arc<Pool<Postgres>>, String> {
        match sqlx::PgPool::connect(db_url).await {
            Ok(pool) => {
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .map_err(|e| format!("{}{}", "Database migration error: ".bright_red(), e))?;
                Ok(Arc::new(pool))
            }
            Err(e) => Err(format!("{}{}", "Database error: ".bright_red(), e))
        }
    }
}
