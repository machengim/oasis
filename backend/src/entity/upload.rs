use crate::util::query::Query;
use crate::{args, util};
use anyhow::{anyhow, Result};
use async_std::fs::{self, OpenOptions};
use async_std::io::prelude::WriteExt;
use std::path::PathBuf;
use util::env;

#[derive(Debug, Clone)]
pub struct UploadTask {
    pub filename: String,
    pub path: String,
    pub file_type: String,
    pub upload_id: String,
    pub parent_id: i64,
    pub size: u64,
    pub current_index: u64,
    pub owner_id: i64,
}

impl UploadTask {
    pub async fn combine_slices(&mut self, storage: &str) -> Result<()> {
        let upload_tmp_dir = env::get_tmp_dir(storage).join(&self.upload_id);
        if !upload_tmp_dir.exists() {
            return Err(anyhow!("Upload tmp dir not exist for {}", self.upload_id));
        }

        let files_dir = env::get_files_dir(storage);
        if !files_dir.exists() {
            fs::create_dir_all(&files_dir).await?;
        }

        let target_filename = get_valid_filename(&files_dir, &self.filename)?;
        match target_filename
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
        {
            Ok(s) => self.path = s,
            Err(e) => {
                return Err(anyhow!("Cannot convert path to string: {:?}", e));
            }
        }
        // TODO: Check Pathbuf to string conversion on non-utf8 OS.
        let mut target_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(target_filename)
            .await?;

        for i in 0..self.current_index {
            let src_file = upload_tmp_dir.join(i.to_string());
            let src_content = fs::read(src_file).await?;
            target_file.write_all(&src_content).await?;
        }

        target_file.sync_all().await?;

        fs::remove_dir_all(upload_tmp_dir).await?;

        Ok(())
    }

    pub fn insert_file_query(&self) -> Result<Query<'_>> {
        let sql = "insert into FILE (filename, file_type, path, size, owner_id, parent_id) values (?1, ?2, ?3, ?4, ?5, ?6)";
        let query = Query::new(
            sql,
            args![
                &self.filename,
                &self.file_type,
                &self.path,
                self.size,
                self.owner_id,
                self.parent_id
            ],
        );

        Ok(query)
    }
}

fn get_valid_filename(dir: &PathBuf, filename: &str) -> Result<PathBuf> {
    let mut path = dir.join(filename);
    let mut index: u64 = 0;
    let split: Vec<&str> = filename.rsplitn(2, '.').collect();

    while path.exists() {
        let new_filename = match split.len() {
            2 => format!("{}-{}.{}", split[1], &index, split[0]),
            1 => format!("{}-{}", split[0], &index),
            _ => return Err(anyhow!("Unknown filename format: {}", filename)),
        };
        path = dir.join(new_filename);
        index += 1;
    }

    Ok(path)
}
