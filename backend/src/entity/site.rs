use crate::args;
use crate::util::{
    self,
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
    pub version: String,
    pub storage: String,
    pub secret: String,
    pub language: String,
    pub update_freq: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct SiteResponse {
    pub version: Option<String>,
    pub storage: Option<String>,
    pub language: Option<String>,
    pub update_freq: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Site {
    pub fn new(storage: &PathBuf, language: &str, created_at: i64) -> Self {
        let secret = util::generate_secret_key();
        let version = util::get_version();
        let storage_str = storage.to_str().unwrap().to_owned();
        let update_freq = util::get_update_freq();

        Self {
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

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> anyhow::Result<i64> {
        let sql = "insert into SITE (version, storage, secret, created_at, language, update_freq, updated_at) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
        let query = Query::new(
            sql,
            args![
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
        let sql = "update SITE set version = ?1, storage = ?2, secret = ?3, created_at = ?4, language = ?5, update_freq = ?6, updated_at = ?7";
        let query = Query::new(
            sql,
            args![
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

impl SiteResponse {
    pub fn from_site(site: &Site, mode: &str) -> AnyResult<Self> {
        match mode {
            "brief" => Ok(Self::from_site_brief(site)),
            "full" => Ok(Self::from_site_full(site)),
            _ => Err(anyhow::anyhow!("Unknown mode to convert site response")),
        }
    }

    fn from_site_brief(site: &Site) -> Self {
        let mut site_res = SiteResponse::default();
        site_res.version = Some(site.version.clone());
        site_res.language = Some(site.language.clone());

        site_res
    }

    fn from_site_full(site: &Site) -> Self {
        let mut site_res = SiteResponse::default();
        site_res.version = Some(site.version.clone());
        site_res.language = Some(site.language.clone());
        site_res.storage = Some(site.storage.clone());
        site_res.created_at = Some(site.created_at);
        site_res.update_freq = Some(site.update_freq.clone());
        site_res.updated_at = Some(site.updated_at);

        site_res
    }
}
