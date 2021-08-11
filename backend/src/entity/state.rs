use super::site::Site;
use anyhow::Result;
use sqlx::{pool::PoolConnection, Pool, Sqlite};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct State {
    pub first_run: Arc<Mutex<bool>>,
    pub pool: Pool<Sqlite>,
    pub storage: Arc<Mutex<String>>,
    pub secret: Arc<Mutex<String>>,
    // pub tasks: Arc<Vec<String>>
}

impl State {
    pub async fn init(pool: Pool<Sqlite>) -> Self {
        let site = Site::init_read(&pool).await;

        Self {
            first_run: Arc::new(Mutex::new(site.first_run == 1)),
            pool,
            storage: Arc::new(Mutex::new(site.storage)),
            secret: Arc::new(Mutex::new(site.secret)),
        }
    }

    pub fn get_first_run(&self) -> Result<bool> {
        let first_run = match self.first_run.lock() {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Cannot retrieve first_run from state: {}",
                    e
                ))
            }
        };

        Ok(*first_run)
    }

    pub async fn get_pool_conn(&self) -> Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }
}
