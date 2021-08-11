use super::site::Site;
use sqlx::{Pool, Sqlite};
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
        let site = Site::read(&pool).await;
        let first_run = site.first_run == 1;

        Self {
            first_run: Arc::new(Mutex::new(first_run)),
            pool,
            storage: Arc::new(Mutex::new(site.storage)),
            secret: Arc::new(Mutex::new(site.secret)),
        }
    }

    pub fn get_first_run(&self) -> anyhow::Result<bool> {
        let first_run = match self.first_run.lock() {
            Ok(v) => v,
            Err(e) => return Err(anyhow::anyhow!("Cannot retrieve first_run from state.")),
        };

        Ok(*first_run)
    }
}
