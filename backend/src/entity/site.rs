use super::query::Query;
use crate::util::db;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};

#[derive(Serialize, FromRow, Debug)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub secret: String,
    pub storage: String,
}

impl Site {
    pub async fn read(pool: &Pool<Sqlite>) -> Self {
        let query = Query::new("SELECT * FROM site", vec![]);
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(_) => panic!("Cannot get db connection"),
        };
        match db::fetch_single::<Site>(query, &mut conn).await {
            Ok(site) => site,
            Err(e) => panic!("Cannot read site info from db: {}", e),
        }
    }
}
