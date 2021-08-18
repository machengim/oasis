pub mod db;
pub mod env;
pub mod file_system;
pub mod query;
use anyhow::Result;
use async_std::fs;
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret_key() -> String {
    let secret_length = env::must_get_env_value("SECRET_LENGTH", 32);

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(secret_length)
        .map(char::from)
        .collect()
}

// TODO: make this function robust.
pub async fn prepare_tmp_dir(storage: &str) -> Result<String> {
    let upload_id = uuid::Uuid::new_v4().to_string();
    let upload_tmp_dir = env::get_tmp_dir(storage).join(&upload_id);
    fs::create_dir_all(upload_tmp_dir).await?;

    Ok(upload_id)
}

pub fn infer_file_type(filename: &str) -> String {
    let splits: Vec<&str> = filename.rsplitn(2, ".").collect();

    if splits.len() <= 1 {
        return "Unknown".to_string();
    }

    let result = match &splits[0].to_lowercase()[..] {
        "c" | "cpp" | "js" | "ts" | "rs" | "py" | "java" | "html" | "css" => "Code",
        "jpg" | "jpeg" | "gif" | "png" => "Image",
        "mp3" | "flac" | "aac" | "ogg" => "Music",
        "pdf" => "Pdf",
        "mp4" | "mov" | "avi" | "mkv" => "Video",
        "txt" => "Text",
        _ => "Unkown",
    };

    result.to_string()
}
