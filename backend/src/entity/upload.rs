use crate::entity::query::Query;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PreUploadRequest {
    pub filename: String,
    pub size: u64,
}

// TODO: implement this function.
impl PreUploadRequest {
    pub fn insert_sql(&self) {}
}
