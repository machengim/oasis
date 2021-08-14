use super::state::State;
use crate::util;
use anyhow::Result;
use async_std::fs;
use serde::Deserialize;
use tide::Request;

#[derive(Deserialize)]
pub struct BeforeUploadRequest {
    pub filename: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct UploadTask {
    pub filename: String,
    pub uuid: String,
    pub size: u64,
    pub current_index: u64,
    pub owner_id: i64,
}

impl BeforeUploadRequest {
    pub fn create_task(&self, uuid: String, owner_id: i64) -> UploadTask {
        UploadTask {
            filename: self.filename.clone(),
            size: self.size,
            uuid,
            current_index: 0,
            owner_id,
        }
    }

    // TODO: make this function robust.
    pub async fn prepare_tmp_dir(req: &Request<State>) -> Result<String> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let upload_tmp_dir = util::get_tmp_dir(req)?.join(&uuid);
        fs::create_dir_all(upload_tmp_dir).await?;

        Ok(uuid)
    }
}
