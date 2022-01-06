use crate::util::db::{self, Query};
use crate::{args, util};
use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::{FromRow, Sqlite, Transaction};
use tokio::fs;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResetPassword {
    pub reset_id: String,
    pub username: String,
    pub expire_at: i64,
}

impl ResetPassword {
    pub fn new(username: &str) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        let expire_at = util::get_utc_seconds() + 60 * 60 * 6;

        Self {
            reset_id: uuid,
            username: username.to_string(),
            expire_at,
        }
    }

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        // One user can only have one reset password record.
        Self::delete_query(&self.username, tx).await?;

        let sql = "insert into RESET (reset_id, username, expire_at) values (?1, ?2, ?3)";
        let query = Query::new(sql, args![&self.reset_id, &self.username, self.expire_at]);

        let uid = db::execute(query, tx).await?;
        Ok(uid)
    }

    pub async fn delete_query(username: &str, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<()> {
        let sql = "delete from RESET where username = ?1";
        let query = Query::new(sql, args![username]);

        db::execute(query, tx).await?;
        Ok(())
    }

    pub async fn remove_user_reset_password_files(
        &self,
        conn: &mut PoolConnection<Sqlite>,
    ) -> AnyResult<()> {
        let sql = "select * from RESET where username = ?1";
        let query = Query::new(sql, args![self.username]);
        let resets: Vec<ResetPassword> = db::fetch_multiple(query, conn).await?;
        let temp_path = util::get_data_temp_path();

        for reset in resets {
            let file_path = temp_path.join(format!("{}.txt", reset.reset_id));
            if file_path.exists() {
                fs::remove_file(&file_path).await?;
            }
        }

        Ok(())
    }

    pub async fn write_reset_password_file(&self, url: &str) -> AnyResult<()> {
        let temp_path = util::get_data_temp_path();
        if !temp_path.exists() {
            fs::create_dir_all(&temp_path).await?;
        }

        let file_name = format!("{}.txt", self.reset_id);
        let file_path = temp_path.join(&file_name);
        let content =
            format!("Dear {}, please visit the following link to reset your password before it expires:\n{}/reset-password/{}", self.username, url, self.reset_id);
        util::file_system::write_text_file(&file_path, &content).await?;

        Ok(())
    }
}
