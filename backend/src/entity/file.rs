use crate::entity::query::Query;
use crate::util::db;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct File {
    pub file_id: i64,
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub file_type: String,
    pub owner_id: i64,
    pub parent_id: i64,
    pub created_at: String,
    pub last_modified_at: String,
}

impl File {
    pub async fn get_files_in_dir(
        dir: i64,
        owner: i64,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<File>> {
        println!("dir_id is {} and owner_id is {}", dir, owner);
        let sql = "select * from FILE where parent_id=?1 and owner_id=?2";
        let query = Query::from(sql, vec![dir.to_string(), owner.to_string()]);

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
