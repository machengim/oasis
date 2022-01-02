use crate::{
    args,
    util::db::{self, Query},
};
use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
use sqlx::{pool::PoolConnection, FromRow, Sqlite, Transaction};

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Hidden {
    pub hidden_id: i64,
    pub path: String,
    pub least_permission: i8,
}

impl Hidden {
    pub fn new(path: &str, least_permission: i8) -> Self {
        Self {
            hidden_id: 0,
            path: String::from(path),
            least_permission,
        }
    }

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        let sql = "insert into HIDDEN (path, least_permission) values (?1, ?2)";
        let query = Query::new(sql, args![&self.path, self.least_permission]);

        let uid = db::execute(query, tx).await?;
        Ok(uid)
    }

    pub async fn delete_query(path: &str, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<()> {
        let sql = "delete from HIDDEN where path = ?1";
        let query = Query::new(sql, args![path]);

        db::execute(query, tx).await?;
        Ok(())
    }

    pub async fn delete_all_query(tx: &mut Transaction<'_, Sqlite>) -> AnyResult<()> {
        let sql = "delete from HIDDEN";
        let query = Query {
            sql,
            args: Vec::new(),
        };

        db::execute(query, tx).await?;
        Ok(())
    }

    // Update the record with the exact path
    // Then update all records (child files) start with `path + "/"`
    pub async fn update_all_sub_path_query(
        tx: &mut Transaction<'_, Sqlite>,
        current_path: &str,
        new_path: &str,
    ) -> AnyResult<()> {
        let sql = "update HIDDEN set path = ?1 where path = ?2";
        let query = Query::new(sql, args![current_path, new_path]);

        db::execute(query, tx).await?;

        // update hidden set path = replace(substr(path, 1, 7), 'alpine/', 'alpine1/') || substr(path, 8) where path like "alpine/%";
        let sql =
            "update HIDDEN set path = replace(substr(path, ?1, ?2), ?3, ?4) || substr(path, ?5) where path like ?6";
        let query = Query::new(
            sql,
            args![
                1,
                current_path.len() + 2,
                format!("{}/", current_path),
                format!("{}/", new_path),
                current_path.len() + 3,
                format!("{}/%", current_path)
            ],
        );

        db::execute(query, tx).await?;

        Ok(())
    }

    pub async fn delete_all_sub_path_query(
        tx: &mut Transaction<'_, Sqlite>,
        path: &str,
    ) -> AnyResult<()> {
        let sql = "delete from HIDDEN where path = ?1";
        let query = Query::new(sql, args![path]);
        db::execute(query, tx).await?;

        let sql = "delete from HIDDEN where path like ?1";
        let query = Query::new(sql, args![format!("{}/%", path)]);
        db::execute(query, tx).await?;

        Ok(())
    }

    pub async fn find_all(conn: &mut PoolConnection<Sqlite>) -> AnyResult<Vec<Self>> {
        let sql = "select * from HIDDEN";
        let query = Query {
            sql,
            args: Vec::new(),
        };

        let hiddens = db::fetch_multiple::<Self>(query, conn).await?;
        Ok(hiddens)
    }
}
