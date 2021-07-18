use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub permission: i8,
    pub created_at: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct UserId {
    pub user_id: i32,
}
