#[derive(Debug, Clone)]
pub struct UploadTask {
    pub filename: String,
    pub path: String,
    pub file_type: String,
    pub upload_id: String,
    pub parent_id: i64,
    pub size: i64,
    pub current_index: u64,
    pub owner_id: i64,
    pub last_modified_at: i64,
}
