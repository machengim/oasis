use crate::{args, service::state::State, util::query::Query};
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use tide::Request;

#[derive(Deserialize, Debug)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
    pub time: Option<i64>,
}

impl SetupRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        let mut setup_req: Self = req.body_json().await?;
        setup_req.time = Some(chrono::Utc::now().timestamp_millis());

        Ok(setup_req)
    }

    pub fn validate(&self) -> Result<bool> {
        if self.username.len() < 1 || self.password.len() < 6 || !Path::new(&self.storage).exists()
        {
            eprintln!("Invalid request data: {:?}", self);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn init_site_query(&self, secret: &str) -> Query {
        let now = self.time.unwrap();
        let sql = "insert into SITE (version, first_run, storage, secret, created_at) values (?1, ?2, ?3, ?4, ?5)";
        Query::new(sql, args![0.1, 0, &self.storage, secret, now])
    }

    // pub fn init_admin_query<'a>(&self) -> Result<Query<'a>> {
    //     let mut user = User::default();
    //     user.username = self.username.clone();
    //     user.password = self.password.clone();
    //     user.permission = 9;

    //     Ok(user.insert_user_query()?)
    // }

    pub fn prepare_root_in_db_query(&self, time: i64) -> Query {
        let sql = "insert into FILE(filename, size, file_type, owner_id, parent_id, last_modified_at) values(?1, ?2, ?3, (select user_id from USER where username = ?4), ?5, ?6)";
        Query::new(sql, args!["root", 0, "root", &self.username, 0, time])
    }
}
