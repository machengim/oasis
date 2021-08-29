pub mod db;
pub mod file_system;
pub mod init;
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
