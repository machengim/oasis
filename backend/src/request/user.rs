use crate::service::state::State;
use serde::Deserialize;
use tide::Request;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl LoginRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        Ok(req.body_json().await?)
    }

    pub fn validate(&self) -> bool {
        self.username.len() >= 2 && self.password.len() >= 6
    }
}
