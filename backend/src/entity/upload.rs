use crate::entity::query::Query;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct PreUploadRequest {
    pub filename: String,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct SliceUploadRequest {
    pub upload_id: String,
    pub index: u64,
    pub data: Vec<u8>,
    pub hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Upload {
    pub upload_id: String,
    pub filename: String,
    pub size: u64,
    pub owner: i64,
}

pub struct UploadState {
    pub uploads: Mutex<Vec<Upload>>,
}

// TODO: implement this function.
impl Upload {
    pub fn from(pre_upload: PreUploadRequest, user_id: i64) -> Self {
        let upload_id = uuid::Uuid::new_v4().to_string();
        Upload {
            upload_id,
            filename: pre_upload.filename,
            size: pre_upload.size,
            owner: user_id,
        }
    }
}

impl UploadState {
    pub fn new() -> Self {
        let uploads = vec![];
        UploadState {
            uploads: Mutex::new(uploads),
        }
    }
}
