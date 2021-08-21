use crate::util::{db, query::Query};
use serde::Serialize;
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: i64,
    pub secret: String,
    pub storage: String,
}

impl Site {
    fn default() -> Self {
        Self {
            version: 0.1,
            first_run: 1,
            created_at: 0,
            secret: String::new(),
            storage: String::new(),
        }
    }
    // The pool value is only passed at the initialization of app.
    // Later on the pool connection has to be acquired from app state.
    pub async fn init_read(pool: &Pool<Sqlite>) -> Self {
        let query = Query::new("SELECT * FROM site", vec![]);
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(_) => panic!("Cannot get db connection"),
        };
        match db::fetch_single::<Site>(query, &mut conn).await {
            Ok(Some(site)) => site,
            _ => Site::default(),
        }
    }
}
