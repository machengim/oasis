pub mod env;
pub mod file_system;
pub mod init;
use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret_key() -> String {
    let secret_length = env::must_get_env_value("SECRET_LENGTH", 32);

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(secret_length)
        .map(char::from)
        .collect()
}

pub fn get_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn split_dir_string(dir_str: &str) -> Result<Vec<String>> {
    let dir_decode = urlencoding::decode(dir_str)?;
    let dir_split: Vec<String> = dir_decode
        .to_string()
        .split("/")
        .map(|s| match urlencoding::decode(s) {
            Ok(v) => v.to_string(),
            _ => "".to_owned(),
        })
        .collect();

    Ok(dir_split)
}
