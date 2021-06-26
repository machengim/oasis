use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Config {
    version: f64,
    first_run: u8,
    //pub port: u32,
    allow_guest: u8,
    default_user_group: u32,
    language: u32
}

#[derive(Serialize, FromRow, Debug)]
pub struct Language {
    id: u32,
    code: String,
    language_name: String
}

#[derive(Serialize, FromRow, Debug)]
pub struct Group {
    id: u32,
    group_name: String,
    power: u8,
    status: u8
}

#[derive(Serialize, FromRow, Debug)]
pub struct Category {
    id: u32,
    category_name: String,
    permission: u8,
    seq: u8
}