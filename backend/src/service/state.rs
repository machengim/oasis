use crate::entity::{site::Site, upload::UploadTask};
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

    pub async fn get_pool_conn(&self) -> Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
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

    pub fn get_storage(&self) -> Result<String> {
        let site = self.get_site()?;

        Ok(site.storage.clone())
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

    pub fn find_upload_task_id(&self, upload_id: &str) -> Result<UploadTask> {
        let tasks = self.get_tasks()?;
        let task_found = tasks.iter().find(|&t| t.upload_id == upload_id).unwrap();

        Ok(task_found.clone())
    }

    pub fn add_task(&self, task: UploadTask) -> Result<()> {
        let mut tasks = self.get_tasks()?;
        if let Some(_) = tasks.iter().find(|&t| t.upload_id == task.upload_id) {
            return Err(anyhow!("Task upload_id already existed in state"));
        }

        tasks.push(task);

        Ok(())
    }

    // TODO: remove unused task from state after a specific time, eg.1 hour.
    pub fn remove_task(&self, task: UploadTask) -> Result<()> {
        let mut tasks = self.get_tasks()?;
        tasks.retain(|t| t.upload_id != task.upload_id);

        Ok(())
    }

    pub fn update_task_index(&self, upload_id: &str) -> Result<()> {
        let mut tasks = self.get_tasks()?;
        let task = tasks.iter_mut().find(|t| t.upload_id == upload_id).unwrap();
        task.current_index += 1;

        Ok(())
    }
}
