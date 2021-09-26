use crate::{
    api::sys::SetupRequest,
    args,
    service::token::{AccessToken, RefreshToken},
    util::db,
    util::db::Query,
};
use anyhow::Result as AnyResult;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{pool::PoolConnection, FromRow, Sqlite, Transaction};

#[derive(FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: i8,
    pub created_at: i64,
}

impl User {
    pub fn from_setup_req(req: &SetupRequest, created_at: i64) -> Self {
        Self {
            user_id: 0,
            username: req.username.to_string(),
            password: req.password.to_string(),
            permission: 9,
            created_at,
        }
    }

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;

        let sql =
            "insert into USER (username, password, permission, created_at) values(?1, ?2, ?3, ?4)";
        let query = Query::new(
            sql,
            args![
                &self.username,
                &encrypt_password,
                self.permission,
                self.created_at
            ],
        );

        let uid = db::execute(query, tx).await?;

        Ok(uid)
    }

    pub async fn update(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;

        let sql =
            "update USER set username = ?1, password = ?2, permission = ?3, created_at = ?4 where user_id = ?5";
        let query = Query::new(
            sql,
            args![
                &self.username,
                &encrypt_password,
                self.permission,
                self.created_at,
                self.user_id
            ],
        );

        let uid = db::execute(query, tx).await?;

        Ok(uid)
    }

    pub async fn find_user_by_name(
        username: &str,
        conn: &mut PoolConnection<Sqlite>,
    ) -> AnyResult<Option<Self>> {
        let sql = "select * from USER where username = ?1";
        let query = Query::new(sql, args![username]);

        Ok(db::fetch_single(query, conn).await?)
    }

    pub async fn find_user_by_id(
        uid: i64,
        conn: &mut PoolConnection<Sqlite>,
    ) -> AnyResult<Option<Self>> {
        let sql = "select * from USER where user_id = ?1";
        let query = Query::new(sql, args![uid]);

        Ok(db::fetch_single(query, conn).await?)
    }

    pub async fn login(
        username: &str,
        password: &str,
        conn: &mut PoolConnection<Sqlite>,
    ) -> AnyResult<Self> {
        let user = match Self::find_user_by_name(username, conn).await? {
            Some(u) => u,
            _ => return Err(anyhow::anyhow!("No such username in db")),
        };

        if !verify(password, &user.password)? {
            return Err(anyhow::anyhow!("Invalid password to login"));
        }

        Ok(user)
    }

    pub fn generate_access_token(&self) -> AccessToken {
        AccessToken::new(self.user_id, self.permission)
    }

    pub fn generate_refresh_token(&self) -> RefreshToken {
        RefreshToken::new(self.user_id)
    }
}
