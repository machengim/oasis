use crate::entity::query::Query;
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

pub fn setup_site_sql(storage: &str) -> Query {
    Query::from(
        "update SITE set first_run = ?1, storage = ?2",
        vec!["0", storage.into()],
    )
}
