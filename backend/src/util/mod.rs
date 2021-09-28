pub mod constants;
pub mod db;
pub mod file_system;
pub mod init;
pub mod rocket_env;
use anyhow::Result as AnyResult;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn generate_secret_key() -> String {
    let secret_length = 32;

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(secret_length)
        .map(char::from)
        .collect()
}

pub fn parse_encoded_url(url: &str) -> AnyResult<PathBuf> {
    let url_decode = urlencoding::decode(url)?;

    Ok(PathBuf::from(url_decode.into_owned()))
}

pub fn get_frontend_path() -> PathBuf {
    let front_dir = get_front_dir_constant();
    let path = PathBuf::from(front_dir);

    if !path.exists() || !path.is_dir() {
        panic!("Invalid frontend directory");
    }

    path
}

pub fn get_version_constant() -> String {
    constants::VERSION.to_string()
}

pub fn sha256(input: &str, secret: &str) -> String {
    let mut hasher = Sha256::new();
    let input_with_secret = format!("{}&key={}", input, secret);
    hasher.update(input_with_secret);
    let result = hasher.finalize();
    format!("{:X}", &result)
}

pub fn get_utc_seconds() -> i64 {
    chrono::Utc::now().timestamp()
}

#[cfg(debug_assertions)]
pub fn get_front_dir_constant() -> String {
    constants::FRONTEND_DIR_DEBUG.to_string()
}

#[cfg(not(debug_assertions))]
pub fn get_front_dir_constant() -> String {
    constants::FRONTEND_DIR_RELEASE.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha256() {
        let input = "Hello world";
        let sha_result = sha256(input, "12ab34cd");
        println!("sha result is {}", &sha_result);
        assert!(sha_result.len() > 0);
    }
}
