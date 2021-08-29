use crate::args;
use crate::service::db;
use crate::service::query::Query;
use crate::util;
use anyhow::{anyhow, Result};
use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, Transaction, Type};
use std::fmt;

#[derive(Clone, Serialize, Type, Debug, PartialEq)]
pub enum FileType {
    Dir,
    Code,
    Text,
    Pdf,
    Image,
    Music,
    Video,
    Unknown,
}

impl Default for FileType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Serialize, FromRow, Debug, Default, Clone)]
pub struct File {
    pub file_id: i64,
    pub filename: String,
    pub size: i64,
    pub file_type: FileType,
    pub owner_id: i64,
    pub parent_id: i64,
    pub last_modified_at: i64,
}

impl fmt::Display for FileType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Dir => fmt.write_str("Dir")?,
            FileType::Code => fmt.write_str("Code")?,
            FileType::Text => fmt.write_str("Text")?,
            FileType::Pdf => fmt.write_str("Pdf")?,
            FileType::Image => fmt.write_str("Image")?,
            FileType::Music => fmt.write_str("Music")?,
            FileType::Video => fmt.write_str("Video")?,
            _ => fmt.write_str("Unknown")?,
        }
        Ok(())
    }
}

impl FileType {
    pub fn infer_file_type(ext: &str) -> Self {
        match ext {
            "c" | "cpp" | "js" | "ts" | "rs" | "py" | "java" | "html" | "css" => Self::Code,
            "jpg" | "jpeg" | "gif" | "png" => Self::Image,
            "mp3" | "flac" | "aac" | "ogg" => Self::Music,
            "pdf" => Self::Pdf,
            "mp4" | "mov" | "avi" | "mkv" => Self::Video,
            "txt" => Self::Text,
            _ => Self::Unknown,
        }
    }
}

impl File {
    pub async fn create_query(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<i64> {
        let sql = "insert into File (filename, size, file_type, owner_id, parent_id, last_modified_at) values(?1, ?2 ,?3, ?4, ?5, ?6)";
        let query = Query::new(
            sql,
            args![
                self.filename,
                self.size,
                self.file_type,
                self.owner_id,
                self.parent_id,
                self.last_modified_at
            ],
        );

        let id = db::tx_execute(query, tx).await?;
        Ok(id)
    }

    pub async fn read_file_by_id(file_id: i64, conn: &mut PoolConnection<Sqlite>) -> Result<Self> {
        let sql = "select * from File where file_id = ?1";
        let query = Query::new(sql, args![file_id]);
        let file = match db::fetch_single(query, conn).await? {
            Some(v) => v,
            None => return Err(anyhow!("File not found")),
        };

        Ok(file)
    }

    pub fn create_sub_dir(&self, filename: &str) -> Self {
        Self {
            file_id: 0,
            filename: String::from(filename),
            file_type: FileType::Dir,
            parent_id: self.file_id,
            owner_id: self.owner_id,
            size: 0,
            last_modified_at: util::get_timestamp(),
        }
    }

    async fn read_user_root_dir(user_id: i64, conn: &mut PoolConnection<Sqlite>) -> Result<Self> {
        let sql = "select * from File where owner_id = ?1 and parent_id = ?2";
        let query = Query::new(sql, args![user_id, 0]);
        let root_dir: Self = match db::fetch_single(query, conn).await? {
            Some(v) => v,
            _ => return Err(anyhow!("Cannot find user root dir")),
        };

        Ok(root_dir)
    }

    pub async fn get_dir_by_path(
        user_id: i64,
        path: &Vec<String>,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<Self>> {
        let mut dir = Self::read_user_root_dir(user_id, conn).await?;
        for p in path.iter() {
            let sql = "select * from File where owner_id = ?1 and parent_id = ?2 and filename = ?3";
            let query = Query::new(sql, args![user_id, dir.file_id, p]);
            dir = match db::fetch_single(query, conn).await? {
                Some(v) => v,
                _ => return Ok(None),
            };
        }

        if dir.file_type != FileType::Dir {
            return Ok(None);
        }

        Ok(Some(dir))
    }

    pub async fn read_dir_contents(
        &self,
        user_id: i64,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>> {
        let sql = "select * from File where owner_id = ?1 and parent_id = ?2";
        let query = Query::new(sql, args![user_id, self.file_id]);

        Ok(db::fetch_multiple(query, conn).await?)
    }
}
