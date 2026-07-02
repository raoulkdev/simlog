use std::sync::Arc;

use sqlx::{Pool, Postgres};

pub struct Server {
    address: String,
    database: Arc<Pool<Postgres>>
}

impl Server {
    
    
}