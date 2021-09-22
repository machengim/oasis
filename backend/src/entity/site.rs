use crate::args;
use crate::util::{
    self,
    constants::DEFAULT_UPDATE_FREQ,
    db::{self, fetch_single, Query},
};
use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
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

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SiteBriefResponse {
    pub name: String,
    pub version: String,
    pub language: String,
    pub update_freq: String,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SiteFullResponse {
    pub name: String,
    pub version: String,
    pub language: String,
    pub update_freq: String,
    pub storage: String,
}

impl Site {
    pub fn new(name: &str, storage: &PathBuf, language: &str, created_at: i64) -> Self {
        let secret = util::generate_secret_key();
        let version = util::get_version();
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
}

impl From<Site> for SiteBriefResponse {
    fn from(s: Site) -> Self {
        Self {
            name: s.name,
            version: s.version,
            language: s.language,
            update_freq: s.update_freq,
        }
    }
}

impl Default for SiteBriefResponse {
    fn default() -> Self {
        Self {
            name: "Oasis".to_owned(),
            version: "0.1".to_owned(),
            language: "en".to_owned(),
            update_freq: "monthly".to_owned(),
        }
    }
}

impl From<Site> for SiteFullResponse {
    fn from(s: Site) -> Self {
        Self {
            name: s.name,
            version: s.version,
            language: s.language,
            storage: s.storage,
            update_freq: s.update_freq,
        }
    }
}

impl Default for SiteFullResponse {
    fn default() -> Self {
        Self {
            name: "Oasis".to_owned(),
            version: "0.1".to_owned(),
            language: "en".to_owned(),
            storage: String::new(),
            update_freq: "monthly".to_owned(),
        }
    }
}
