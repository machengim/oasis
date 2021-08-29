use crate::{
    service::{state::State, token::Token},
    util,
};
use anyhow::{anyhow, Result};
use async_std::fs;
use serde::Deserialize;
use tide::Request;

#[derive(Deserialize, Default, Debug)]
pub struct GetDirRequest {
    // pub path: PathBuf,
    pub path: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct CreateDirRequest {
    // pub path: PathBuf,
    pub paths: Vec<String>,
    pub dir_name: String,
}

impl GetDirRequest {
    pub async fn from_req(req: &mut Request<State>) -> Result<Self> {
        let mut result = Self::default();

        if let Ok(dir_path) = req.param("dir_path") {
            result.path = util::split_dir_string(dir_path)?;
        }

        Ok(result)
    }
}

impl CreateDirRequest {
    pub async fn from_req(req: &mut Request<State>) -> tide::Result<Self> {
        req.body_json().await
    }

    pub fn validate(&self) -> bool {
        self.dir_name.len() > 0
    }

    pub async fn create_dir(&self, req: &mut Request<State>) -> Result<()> {
        let storage = req.state().get_storage()?;
        let username = Token::from_ext(req)?.username;
        let dir = util::file_system::get_required_dir(&storage, &username, &self.paths);

        if !dir.exists().await || !dir.is_dir().await {
            return Err(anyhow!("Parent directory not found"));
        }

        let sub_dir = dir.join(&self.dir_name);

        if sub_dir.exists().await {
            return Err(anyhow!("Sub directory already existed"));
        }

        Ok(fs::create_dir(sub_dir).await?)
    }
}
