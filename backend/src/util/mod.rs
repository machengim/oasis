pub mod constants;
pub mod db;
pub mod file_system;
pub mod init;
pub mod rocket_env;
use anyhow::Result as AnyResult;
use rand::{distributions::Alphanumeric, Rng};
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

#[cfg(debug_assertions)]
pub fn get_front_dir_constant() -> String {
    constants::FRONTEND_DIR_DEBUG.to_string()
}

#[cfg(not(debug_assertions))]
pub fn get_front_dir_constant() -> String {
    constants::FRONTEND_DIR_RELEASE.to_string()
}
