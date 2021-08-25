use crate::service::state::State;
use anyhow::Result;
use async_std::path::PathBuf;
use tide::Request;

#[derive(Debug)]
pub struct SysDirsRequest {
    pub path: PathBuf,
}

impl SysDirsRequest {
    pub fn from(req: &Request<State>) -> tide::Result<Self> {
        let dir_str = req.param("dir")?;

        let path = urlencoding::decode(dir_str)?;

        Ok(Self {
            path: PathBuf::from(path.to_string()),
        })
    }

    pub async fn validate(&self) -> bool {
        if !self.path.exists().await {
            eprintln!("Invalid path in sys dir request: {:?}", self);
            return false;
        }

        true
    }

    pub fn auth(&self, req: &Request<State>) -> Result<bool> {
        let first_run = req.state().get_first_run()?;

        if !first_run {
            return Ok(false);
        }

        Ok(true)
    }
}
