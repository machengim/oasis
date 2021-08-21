use crate::entity::file::File;
use crate::entity::upload::UploadTask;
use crate::service::state::State;
use crate::service::token::Token;
use crate::util;
use anyhow::{anyhow, Result};
use async_std::fs;
use async_std::io::prelude::WriteExt;
use serde::Deserialize;
use tide::Request;
use util::env;

#[derive(Deserialize, Debug)]
pub struct PrepareUploadRequest {
    pub filename: String,
    pub parent_id: i64,
    pub size: u64,
}

#[derive(Deserialize, Default, Debug)]
pub struct SliceUploadQuery {
    pub index: u64,
    pub hash: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct SliceUploadRequest {
    pub index: u64,
    pub hash: String,
    pub upload_id: String,
    pub data: Vec<u8>,
}

#[derive(Deserialize)]
pub struct FinishUploadRequest {
    pub upload_id: String,
}

impl PrepareUploadRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let upload_req: Self = req.body_json().await?;

        Ok(upload_req)
    }

    pub async fn validate(&self) -> Result<bool> {
        if self.filename.len() == 0 || self.size <= 0 {
            eprintln!("Before upload request format error");
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn auth(&self, req: &Request<State>) -> Result<bool> {
        let token = Token::from_ext(&req)?;
        if token.permission <= 0 {
            eprintln!("User auth failed for upload request");
            return Ok(false);
        }

        let mut conn = req.state().get_pool_conn().await?;
        let folder_owner_id = File::find_file_owner(self.parent_id, &mut conn).await?;
        if folder_owner_id != token.uid {
            eprintln!("Current user and dir owner not match");
            return Ok(false);
        }

        Ok(true)
    }

    pub fn create_task(&self, upload_id: &str, owner_id: i64) -> UploadTask {
        UploadTask {
            filename: self.filename.clone(),
            size: self.size,
            upload_id: String::from(upload_id),
            current_index: 0,
            parent_id: self.parent_id,
            owner_id,
            path: String::new(),
            file_type: util::infer_file_type(&self.filename),
        }
    }
}

impl SliceUploadRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let upload_id = req.param("upload_id")?.to_string();
        let slice_query: SliceUploadQuery = req.query()?;
        let data = req.body_bytes().await?;

        Ok(Self {
            upload_id,
            data,
            index: slice_query.index,
            hash: slice_query.hash,
        })
    }

    pub fn validate(&self, req: &Request<State>) -> Result<bool> {
        let task = req.state().find_upload_task_id(&self.upload_id)?;
        if self.index != task.current_index {
            eprintln!("Slice index conflict");
            return Ok(false);
        }

        if !self.validate_hash() {
            eprintln!("Upload data corrupted");
            return Ok(false);
        }

        Ok(true)
    }

    pub fn auth(&self, req: &Request<State>) -> Result<bool> {
        let token = Token::from_ext(req)?;
        if token.permission <= 0 {
            eprintln!("User auth failed for upload request");
            return Ok(false);
        }

        let task = req.state().find_upload_task_id(&self.upload_id)?;
        if task.owner_id != token.uid {
            eprintln!("Current user and upload task owner not match");
            return Ok(false);
        }

        Ok(true)
    }

    fn validate_hash(&self) -> bool {
        let data_hash = md5::compute(&self.data);

        format!("{:?}", data_hash) == self.hash
    }

    pub async fn write_tmp_file(&self, storage: &str) -> Result<()> {
        let upload_tmp_dir = env::get_tmp_dir(storage).join(&self.upload_id);
        if !upload_tmp_dir.exists() {
            return Err(anyhow!("Upload tmp dir not exist for {}", &self.upload_id));
        }

        let upload_tmp_file = upload_tmp_dir.join(self.index.to_string());
        let mut file = fs::File::create(upload_tmp_file).await?;
        file.write_all(&self.data).await?;

        Ok(())
    }
}

impl FinishUploadRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        Ok(req.body_json().await?)
    }

    pub fn auth(&self, req: &Request<State>) -> Result<bool> {
        if !validate_upload_user(req, &self.upload_id)? {
            return Ok(false);
        }

        Ok(true)
    }
}

fn validate_upload_user(req: &Request<State>, upload_id: &str) -> anyhow::Result<bool> {
    let token = Token::from_ext(req)?;
    if token.permission <= 0 {
        return Ok(false);
    }

    let task = req.state().find_upload_task_id(upload_id)?;
    if task.owner_id != token.uid {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let content = b"Hello world";
        println!("Content: {:?}", content);

        let data: [u8; 12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 10];
        println!("Data: {:?}", &data);

        let hash = md5::compute(&data);
        println!("hashed: {:?}", &hash);
    }

    #[test]
    fn test_hash_lib() {
        let content = b"Hello world";
        let content_hash = md5::compute(&content);

        assert_eq!(
            format!("{:?}", content_hash),
            "3e25960a79dbc69b674cd4ec67a72c62"
        );
    }

    #[test]
    fn test_validate_hash() {
        let data: Vec<u8> = b"Hello world".iter().cloned().collect();

        let slice = SliceUploadRequest {
            index: 1,
            hash: "3e25960a79dbc69b674cd4ec67a72c62".into(),
            data,
            upload_id: String::new(),
        };

        assert!(slice.validate_hash());
    }
}
