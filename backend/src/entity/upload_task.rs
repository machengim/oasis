use super::request::UploadRequest;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct UploadTask {
    pub uuid: String,
    pub userid: i64,
    pub filename: String,
    pub size: u64,
    pub dir: PathBuf,
    pub hash: String,
    pub finished_slices: u64,
}

// Not implementing request guard as the different verification logic
// for uploading and cancelling upload. The second endpoint requires
// a list of task uuids.
impl UploadTask {
    pub fn new(upload_req: &UploadRequest, userid: i64, target_path: PathBuf) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();

        Self {
            uuid,
            userid,
            filename: upload_req.filename.to_owned(),
            size: upload_req.size,
            dir: target_path,
            hash: upload_req.hash.to_owned(),
            finished_slices: 0,
        }
    }
}
