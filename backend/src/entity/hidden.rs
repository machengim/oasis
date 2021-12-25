use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Hidden {
    pub hidden_id: i64,
    pub path: String,
    pub least_permission: i8,
}
