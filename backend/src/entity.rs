use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub storage: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
    pub permission: u8,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Node {
    pub node_id: u32,
    pub node_name: String,
    pub is_dir: u8,
    pub owner_id: u32,
    pub parent_node_id: u32,
    pub created_at: String,
    pub updated_at: String,
}