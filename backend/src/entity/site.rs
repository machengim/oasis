use std::path::PathBuf;

use crate::args;
use crate::util::{
    self,
    db::{self, fetch_single, Query},
};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, Transaction};

#[derive(FromRow, Default, Debug)]
pub struct Site {
    pub site_id: i64,
    pub version: f64,
    pub storage: String,
    pub secret: String,
    pub created_at: i64,
}

impl Site {
    pub fn new(storage: &PathBuf, created_at: i64) -> Self {
        let secret = util::generate_secret_key();
        let storage_str = storage.to_str().unwrap().to_owned();

        Self {
            site_id: 0,
            version: 0.1,
            storage: storage_str,
            secret,
            created_at,
        }
    }

    pub async fn read(conn: &mut PoolConnection<Sqlite>) -> Result<Option<Self>, sqlx::Error> {
        let sql = "select * from Site";
        let query = Query::new(sql, vec![]);

        match fetch_single(query, conn).await? {
            Some(v) => Ok(Some(v)),
            _ => Ok(None),
        }
    }

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> anyhow::Result<i64> {
        let sql = "insert into SITE (version, storage, secret, created_at) values (?1, ?2, ?3, ?4)";
        let query = Query::new(
            sql,
            args![self.version, &self.storage, &self.secret, self.created_at],
        );

        let site_id = db::execute(query, tx).await?;

        Ok(site_id)
    }
}
