use rocket::tokio::{fs, io::AsyncWriteExt};
use serde::Deserialize;
use std::path::Path;
use std::sync::Mutex;

#[derive(Deserialize, Debug)]
pub struct PreUploadRequest {
    pub filename: String,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct SliceUploadRequest {
    pub index: u64,
    pub data: Vec<u8>,
    pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct FinishUploadRequest {
    pub upload_id: String,
    pub filename: String,
    pub slices: u64,
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

impl Upload {
    pub fn from(pre_upload: PreUploadRequest, upload_id: String, user_id: i64) -> Self {
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

impl SliceUploadRequest {
    pub async fn write_to_temp(&self, storage: &str, upload_id: &str) -> anyhow::Result<()> {
        let path = Path::new(storage).join("tmp").join(upload_id);
        if !path.exists() {
            fs::create_dir(&path).await?;
        }

        let filename = path.join(self.index.to_string());
        println!("Writing to file: {:?}", &filename);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .await?;
        file.write_all(&self.data).await?;

        Ok(())
    }
}

impl FinishUploadRequest {
    pub async fn combine_slices(&self, storage: &str) -> anyhow::Result<()> {
        let src_folder = Path::new(storage).join("tmp").join(&self.upload_id);
        if !src_folder.exists() {
            return Err(anyhow::anyhow!(
                "folder for upload {} not exist.",
                &self.upload_id
            ));
        }

        let target_folder = Path::new(storage).join("storage");
        if !target_folder.exists() {
            fs::create_dir(&target_folder).await?;
        }

        let target_filename = target_folder.join(&self.filename);
        println!("target filename: {:?}", &target_filename);
        let mut target_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(target_filename)
            .await?;

        for i in 0..self.slices {
            let source_file = src_folder.join(i.to_string());
            println!("reading source file: {:?}", &source_file);
            let content = fs::read(source_file).await?;
            target_file.write_all(&content).await?;
        }
        Ok(())
    }
}
