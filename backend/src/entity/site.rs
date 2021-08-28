use crate::args;
use crate::service::db;
use crate::service::query::Query;
use anyhow::Result;
use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite, Transaction};

#[derive(Serialize, FromRow, Default, Debug, Clone)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: i64,
    pub secret: String,
    pub storage: String,
}

impl Site {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        let sql = "insert into SITE (version, first_run, created_at, secret, storage) values(?1, ?2, ?3, ?4, ?5)";
        let query = Query::new(
            sql,
            args![
                self.version,
                self.first_run,
                self.created_at,
                &self.secret,
                &self.storage
            ],
        );

        db::tx_execute(query, tx).await?;
        Ok(())
    }

    pub async fn init_read(pool: &Pool<Sqlite>) -> Self {
        let query = Query::new("SELECT * FROM site", vec![]);
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(_) => panic!("Cannot get db connection"),
        };
        match db::fetch_single::<Site>(query, &mut conn).await {
            Ok(Some(site)) => site,
            _ => Site {
                first_run: 1,
                ..Default::default()
            },
        }
    }
}
