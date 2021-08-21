use crate::{args, entity::user::User, service::state::State, util::query::Query};
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use tide::Request;

#[derive(Deserialize, Debug)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
}

impl SetupRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        Ok(req.body_json().await?)
    }

    pub fn validate(&self) -> Result<bool> {
        if self.username.len() < 1 || self.password.len() < 6 || !Path::new(&self.storage).exists()
        {
            eprintln!("Invalid request data: {:?}", self);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn update_site_query(&self, secret: &str) -> Query {
        Query::new(
            "update SITE set first_run = ?1, storage = ?2, secret = ?3",
            args![0, &self.storage, secret],
        )
    }

    pub fn init_admin_query<'a>(&self) -> Result<Query<'a>> {
        let mut user = User::default();
        user.username = self.username.clone();
        user.password = self.password.clone();
        user.permission = 9;

        Ok(user.insert_user_query()?)
    }

    pub fn prepare_root_in_db_query(&self) -> Query {
        let sql = "insert into FILE(filename, size, file_type, owner_id, parent_id) values(?1, ?2, ?3, (select user_id from USER where username = ?4), ?5)";
        Query::new(sql, args!["root", 0, "root", &self.username, 0])
    }
}
