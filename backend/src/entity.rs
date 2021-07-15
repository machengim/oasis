use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};
use std::sync::Mutex;

pub struct AppState {
    pub first_run: Mutex<bool>,
    pub pool: Pool<Sqlite>,
    pub storage: Mutex<String>,
}

pub struct AuthIndex {
    pub option: i8,
}

pub struct AuthDb {
    pub conn: PoolConnection<Sqlite>,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub storage: String,
}

#[derive(Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
}
