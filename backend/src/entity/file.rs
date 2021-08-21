use crate::args;
use crate::request::file::CreateDirRequest;
use crate::util::{db, query::Query};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite};

use super::upload::UploadTask;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct File {
    pub file_id: i64,
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub file_type: String,
    pub owner_id: i64,
    pub parent_id: i64,
    pub last_modified_at: i64,
}

impl File {
    pub fn from_create_dir_req(dir_req: &CreateDirRequest) -> Self {
        let now = chrono::Utc::now().timestamp_millis();

        Self {
            file_id: -1,
            filename: dir_req.dir_name.to_string(),
            path: String::new(),
            size: -1,
            file_type: "Dir".to_string(),
            owner_id: dir_req.user_id.unwrap(),
            parent_id: dir_req.parent_id,
            last_modified_at: now,
        }
    }

    pub fn from_upload_task(task: &UploadTask) -> Self {
        Self {
            file_id: -1,
            filename: task.filename.to_string(),
            path: task.path.to_string(),
            size: task.size,
            file_type: task.file_type.to_string(),
            owner_id: task.owner_id,
            parent_id: task.parent_id,
            last_modified_at: task.last_modified_at,
        }
    }

    pub async fn insert_to_db(mut self, conn: &mut PoolConnection<Sqlite>) -> Result<Self> {
        let query = self.insert_to_db_query()?;
        self.file_id = db::insert_single(query, conn).await?;

        Ok(self)
    }

    pub fn insert_to_db_query(&self) -> Result<Query<'_>> {
        let sql = "insert into FILE (filename, path, size, file_type, owner_id, parent_id, last_modified_at) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
        let query = Query::new(
            sql,
            args!(
                &self.filename,
                &self.path,
                self.size,
                self.file_type,
                self.owner_id,
                self.parent_id,
                self.last_modified_at
            ),
        );

        Ok(query)
    }

    pub async fn get_files_in_dir(
        dir: i64,
        owner: i64,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<File>> {
        println!("dir_id is {} and owner_id is {}", dir, owner);
        let sql = "select * from FILE where parent_id=?1 and owner_id=?2";
        let query = Query::new(sql, args![dir, owner]);

        let files: Vec<File> = db::fetch_multiple(query, conn).await?;

        Ok(files)
    }

    pub async fn get_file_by_id(file_id: i64, conn: &mut PoolConnection<Sqlite>) -> Result<File> {
        let sql = "select * from FILE where file_id = ?1";
        let query = Query::new(sql, vec![file_id.to_string()]);

        match db::fetch_single::<File>(query, conn).await? {
            Some(v) => Ok(v),
            None => Err(anyhow!("No root dir found for user")),
        }
    }

    pub async fn find_root_dir(user_id: i64, conn: &mut PoolConnection<Sqlite>) -> Result<i64> {
        let sql = "select * from FILE where owner_id = ?1 and parent_id = ?2";
        let query = Query::new(sql, vec![user_id.to_string(), 0.to_string()]);

        match db::fetch_single::<File>(query, conn).await? {
            Some(v) => Ok(v.file_id),
            None => Err(anyhow!("No root dir found for user")),
        }
    }

    pub async fn find_file_owner(file_id: i64, conn: &mut PoolConnection<Sqlite>) -> Result<i64> {
        let sql = "select * from FILE where file_id = ?1";
        let query = Query::new(sql, vec![file_id.to_string()]);

        match db::fetch_single::<File>(query, conn).await? {
            Some(v) => Ok(v.owner_id),
            None => Err(anyhow!("No root dir found for user")),
        }
    }
}
