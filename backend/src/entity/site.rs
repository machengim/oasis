use crate::args;
use crate::util::{
    self,
    constants::DEFAULT_UPDATE_FREQ,
    db::{self, fetch_single, Query},
};
use anyhow::Result as AnyResult;
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, Transaction};
use std::path::PathBuf;

#[derive(FromRow, Default, Debug)]
pub struct Site {
    pub site_id: i64,
    pub name: String,
    pub version: String,
    pub storage: String,
    pub secret: String,
    pub language: String,
    pub update_freq: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Site {
    pub fn new(name: &str, storage: &PathBuf, language: &str, created_at: i64) -> Self {
        let secret = util::generate_secret_key();
        let version = util::get_version_constant();
        let storage_str = storage.to_str().unwrap().to_owned();
        let update_freq = DEFAULT_UPDATE_FREQ.to_owned();

        Self {
            name: name.to_string(),
            site_id: 0,
            version,
            storage: storage_str,
            secret,
            language: language.to_string(),
            update_freq,
            created_at,
            updated_at: created_at,
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

    pub async fn insert(&self, tx: &mut Transaction<'_, Sqlite>) -> anyhow::Result<i64> {
        let sql = "insert into SITE (name, version, storage, secret, created_at, language, update_freq, updated_at) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
        let query = Query::new(
            sql,
            args![
                &self.name,
                self.version,
                &self.storage,
                &self.secret,
                self.created_at,
                &self.language,
                &self.update_freq,
                &self.updated_at
            ],
        );

        let site_id = db::execute(query, tx).await?;

        Ok(site_id)
    }

    pub async fn update(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        let sql = "update SITE set name = ?1, version = ?2, storage = ?3, secret = ?4, created_at = ?5, language = ?6, update_freq = ?7, updated_at = ?8";
        let query = Query::new(
            sql,
            args![
                &self.name,
                self.version,
                &self.storage,
                &self.secret,
                self.created_at,
                &self.language,
                &self.update_freq,
                &self.updated_at
            ],
        );

        Ok(db::execute(query, tx).await?)
    }

    pub fn check_update_need(&self) -> bool {
        let day_seconds = 24 * 60 * 60;
        let interval = match self.update_freq.to_lowercase().as_str() {
            "daily" => day_seconds,
            "weekly" => 7 * day_seconds,
            "monthly" => 30 * day_seconds,
            _ => return false,
        };

        let current_timestamp = util::get_utc_seconds();

        current_timestamp - self.updated_at > interval
    }
}
