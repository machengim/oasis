use super::{site::Site, upload::UploadTask};
use anyhow::{anyhow, Result};
use sqlx::{pool::PoolConnection, Pool, Sqlite};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct State {
    pub site: Arc<Mutex<Site>>,
    pub pool: Pool<Sqlite>,
    pub tasks: Arc<Mutex<Vec<UploadTask>>>,
}

impl State {
    pub async fn init(pool: Pool<Sqlite>) -> Self {
        let site = Site::init_read(&pool).await;
        let tasks: Vec<UploadTask> = vec![];

        Self {
            pool,
            site: Arc::new(Mutex::new(site)),
            tasks: Arc::new(Mutex::new(tasks)),
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

    fn get_tasks(&self) -> Result<std::sync::MutexGuard<Vec<UploadTask>>> {
        match self.tasks.lock() {
            Ok(v) => Ok(v),
            Err(e) => return Err(anyhow!("Cannot retrieve tasks from state: {}", e)),
        }
    }

    pub fn find_task_by_uuid(&self, uuid: String) -> Result<UploadTask> {
        let tasks = self.get_tasks()?;
        let tasks_found: Vec<&UploadTask> = tasks.iter().filter(|t| t.uuid == uuid).collect();

        match tasks_found.len() {
            0 => Err(anyhow!("Task uuid not found")),
            _ => Ok(tasks[0].clone()),
        }
    }

    pub fn add_task(&self, task: UploadTask) -> Result<()> {
        let mut tasks = self.get_tasks()?;
        tasks.push(task);

        Ok(())
    }

    pub fn remove_task(&self, task: UploadTask) -> Result<()> {
        let mut tasks = self.get_tasks()?;
        tasks.retain(|t| t.uuid != task.uuid);

        Ok(())
    }
}
