use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Config {
    pub version: f64,
    #[sqlx(rename = "firstRun")]
    pub first_run: u8,
    pub port: u32,
    #[sqlx(rename = "allowGuest")]
    pub allow_guest: u8,
    #[sqlx(rename = "defaultUserGroup")]
    pub default_user_group: u32,
    pub lang: u32
}

#[derive(Serialize, FromRow, Debug)]
pub struct Language {
    id: u32,
    code: String,
    #[sqlx(rename = "langName")]
    lang_name: String
}