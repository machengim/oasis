use crate::args;
use crate::util::{
    self,
    db::{self, fetch_single, Query},
};
use rocket::serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, Transaction};
use std::path::PathBuf;

#[derive(FromRow, Default, Debug)]
pub struct Site {
    pub site_id: i64,
    pub version: String,
    pub storage: String,
    pub secret: String,
    pub language: String,
    pub created_at: i64,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SiteBasic {
    pub version: String,
    pub language: String,
}

impl Site {
    pub fn new(storage: &PathBuf, language: &str, created_at: i64) -> Self {
        let secret = util::generate_secret_key();
        let version = util::get_version();
        let storage_str = storage.to_str().unwrap().to_owned();

        Self {
            site_id: 0,
            version,
            storage: storage_str,
            secret,
            language: language.to_string(),
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
        let sql = "insert into SITE (version, storage, secret, created_at, language) values (?1, ?2, ?3, ?4, ?5)";
        let query = Query::new(
            sql,
            args![
                self.version,
                &self.storage,
                &self.secret,
                self.created_at,
                &self.language
            ],
        );

        let site_id = db::execute(query, tx).await?;

        Ok(site_id)
    }
}

impl SiteBasic {
    pub fn new(language: &str, version: &str) -> Self {
        Self {
            language: language.to_string(),
            version: version.to_string(),
        }
    }
}
