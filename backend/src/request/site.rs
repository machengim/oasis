use crate::entity::site::Site;
use crate::entity::user::User;
use crate::service::state::State;
use crate::util;
use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use tide::Request;

#[derive(Deserialize, Debug)]
pub struct SiteSetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
    pub time: Option<i64>,
}

impl SiteSetupRequest {
    pub async fn from_req(req: &mut Request<State>) -> tide::Result<Self> {
        let mut setup_req: Self = req.body_json().await?;
        setup_req.time = Some(chrono::Utc::now().timestamp_millis());

        let path = urlencoding::decode(&setup_req.storage)?;
        setup_req.storage = path.to_string();

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

    pub fn to_site(&self, secret: &str) -> Site {
        let time = match self.time {
            Some(t) => t,
            None => util::get_timestamp(),
        };

        Site {
            version: 0.1,
            first_run: 0,
            created_at: time,
            secret: secret.to_string(),
            storage: self.storage.to_string(),
        }
    }

    pub fn to_admin(&self) -> User {
        let time = match self.time {
            Some(t) => t,
            None => util::get_timestamp(),
        };

        User {
            user_id: 0,
            username: self.username.to_string(),
            password: self.password.to_string(),
            permission: 9,
            created_at: time,
        }
    }
}
