use super::site::Site;
use anyhow::{anyhow, Result};
use sqlx::{pool::PoolConnection, Pool, Sqlite};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct State {
    pub site: Arc<Mutex<Site>>,
    pub pool: Pool<Sqlite>,
    // pub tasks: Arc<Vec<String>>
}

impl State {
    pub async fn init(pool: Pool<Sqlite>) -> Self {
        let site = Site::init_read(&pool).await;

        Self {
            site: Arc::new(Mutex::new(site)),
            pool,
        }
    }

    fn get_site(&self) -> Result<std::sync::MutexGuard<Site>> {
        match self.site.lock() {
            Ok(v) => Ok(v),
            Err(e) => return Err(anyhow!("Cannot retrieve site from state: {}", e)),
        }
    }

    pub fn get_site_value(&self) -> Result<Site> {
        let site = self.get_site()?;

        Ok((*site).clone())
    }

    pub fn set_site(&self, new_site: Site) -> Result<()> {
        let mut site = self.get_site()?;
        *site = new_site;

        Ok(())
    }

    pub fn get_first_run(&self) -> Result<bool> {
        let site = self.get_site()?;

        Ok(site.first_run == 1)
    }

    pub fn get_secret(&self) -> Result<String> {
        let site = self.get_site()?;

        Ok(site.secret.clone())
    }

    pub fn set_first_run(&self, new_first_run: bool) -> Result<()> {
        let mut site = self.get_site()?;
        site.first_run = if new_first_run { 1 } else { 0 };

        Ok(())
    }

    pub fn set_storage(&self, new_storage: String) -> Result<()> {
        let mut site = self.get_site()?;
        site.storage = new_storage;

        Ok(())
    }

    pub async fn get_pool_conn(&self) -> Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }

    pub fn set_secret(&self, new_secret: String) -> Result<()> {
        let mut site = self.get_site()?;
        site.secret = new_secret;

        Ok(())
    }
}
