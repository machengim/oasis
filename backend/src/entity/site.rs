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

#[derive(Debug)]
pub struct Query<'a> {
    pub sql: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Query<'a> {
    pub fn new(sql: &'a str, args: Vec<&'a str>) -> Self {
        Query { sql, args }
    }
}
