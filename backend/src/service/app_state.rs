use crate::entity::site::Site;
use anyhow::Result as AnyResult;
use sqlx::{pool::PoolConnection, Pool, Sqlite};
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};

#[derive(Debug)]
pub struct AppState {
    pub first_run: AtomicBool,
    pub site: Arc<Mutex<Site>>,
    pub pool: Pool<Sqlite>,
}

impl AppState {
    pub fn new_with_site(site: Site, pool: Pool<Sqlite>) -> Self {
        Self {
            first_run: AtomicBool::new(false),
            site: Arc::new(Mutex::new(site)),
            pool,
        }
    }

    pub fn new_without_site(pool: Pool<Sqlite>) -> Self {
        Self {
            first_run: AtomicBool::new(true),
            site: Arc::new(Mutex::new(Site::default())),
            pool,
        }
    }

    pub fn get_first_run(&self) -> bool {
        self.first_run.load(Ordering::Relaxed)
    }

    pub fn set_first_run(&self, new_first_run: bool) {
        self.first_run.store(new_first_run, Ordering::Relaxed);
    }

    pub fn get_secret(&self) -> AnyResult<String> {
        Ok(self.get_site()?.secret.to_owned())
    }

    pub async fn get_pool_conn(&self) -> Result<PoolConnection<Sqlite>, sqlx::Error> {
        Ok(self.pool.acquire().await?)
    }

    pub fn get_site(&self) -> AnyResult<std::sync::MutexGuard<Site>> {
        match self.site.lock() {
            Ok(v) => Ok(v),
            Err(e) => return Err(anyhow::anyhow!("Cannot retrieve site from state: {}", e)),
        }
    }

    pub fn set_site(&self, new_site: Site) -> AnyResult<()> {
        let mut site = self.get_site()?;
        *site = new_site;

        Ok(())
    }
}
