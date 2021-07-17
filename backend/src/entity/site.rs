use serde::Serialize;
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};
use std::sync::Mutex;

pub struct AppState {
    pub first_run: Mutex<bool>,
    pub pool: Pool<Sqlite>,
    pub storage: Mutex<String>,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub storage: String,
}

pub struct Query {
    pub sql: String,
    pub args: Vec<String>,
}

impl Query {
    pub fn new(sql: &str, args: Vec<String>) -> Self {
        Query {
            sql: String::from(sql),
            args,
        }
    }
}
