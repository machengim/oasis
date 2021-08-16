use crate::entity::query::Query;
use crate::util::db;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct File {
    pub file_id: i64,
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub is_dir: u8,
    pub owner_id: i64,
    pub parent_id: i64,
    pub created_at: String,
    pub updated_at: String,
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
}
