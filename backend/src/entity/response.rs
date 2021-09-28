use crate::service::range::RangedFile;
use rocket::fs::NamedFile;
use rocket::serde::Serialize;

#[derive(Responder)]
pub enum FileResponse {
    Range(RangedFile),
    Binary(NamedFile),
    Text(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppNeedUpdateResponse {
    pub need: bool,
    pub url: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub username: String,
    pub permission: i8,
    pub expire: usize,
}
