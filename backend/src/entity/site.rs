use super::query::Query;
use super::user::User;
use crate::util::{self, db};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub secret: String,
    pub storage: String,
}

#[derive(Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
}

impl Site {
    // The pool value is only passed at the initialization of app.
    // Later on the pool connection has to be acquired from app state.
    pub async fn init_read(pool: &Pool<Sqlite>) -> Self {
        let query = Query::new("SELECT * FROM site", vec![]);
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(_) => panic!("Cannot get db connection"),
        };
        match db::fetch_single::<Site>(query, &mut conn).await {
            Ok(Some(site)) => site,
            Ok(None) => panic!("No site record in db"),
            Err(e) => panic!("Cannot read site info from db: {}", e),
        }
    }

    pub fn default() -> Self {
        let version = util::must_get_env_value("VERSION", 0.1);
        let created_at = chrono::Utc::now().timestamp().to_string();

        Self {
            version,
            first_run: 1,
            created_at,
            secret: String::new(),
            storage: String::new(),
        }
    }
}

impl SetupRequest {
    pub fn update_site_query(&self, secret: &str) -> Query {
        Query::from(
            "update SITE set first_run = ?1, storage = ?2, secret = ?3",
            vec!["0", &self.storage, secret],
        )
    }

    pub fn init_admin_query<'a>(&self) -> Result<Query<'a>> {
        let mut user = User::default();
        user.username = self.username.clone();
        user.password = self.password.clone();
        user.permission = 9;

        Ok(user.insert_user_query()?)
    }
}
