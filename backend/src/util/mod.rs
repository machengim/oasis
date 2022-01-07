pub mod constants;
pub mod db;
pub mod file_system;
pub mod init;
pub mod local_ip;
pub mod rocket_env;
use anyhow::Result as AnyResult;
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn generate_secret_key(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn parse_encoded_url(url: &str) -> AnyResult<PathBuf> {
    let url_decode = urlencoding::decode(url)?;

    Ok(PathBuf::from(url_decode.into_owned()))
}

pub fn get_frontend_path() -> PathBuf {
    let path = get_frontend_dir();

    if !path.exists() || !path.is_dir() {
        panic!("Invalid frontend directory");
    }

    path
}

pub fn get_temp_path() -> PathBuf {
    get_pwd().join("temp")
}

pub fn get_data_temp_path() -> PathBuf {
    get_pwd().join("data").join("temp")
}

#[cfg(debug_assertions)]
pub fn get_frontend_dir() -> PathBuf {
    let front_dir = constants::FRONTEND_DIR_DEBUG;
    PathBuf::from(front_dir)
}

#[cfg(not(debug_assertions))]
pub fn get_frontend_dir() -> PathBuf {
    let front_dir = constants::FRONTEND_DIR_RELEASE;
    let pwd = get_pwd();
    pwd.join(front_dir)
}

#[cfg(debug_assertions)]
pub fn get_verion_url() -> String {
    String::from(constants::APP_VERSION_URL_DEBUG)
}

#[cfg(not(debug_assertions))]
pub fn get_verion_url() -> String {
    String::from(constants::APP_VERSION_URL_RELEASE)
}

pub fn get_pwd() -> PathBuf {
    let exe_file = std::env::current_exe().expect("Cannot get app directory");
    exe_file
        .parent()
        .expect("Cannot get parent dir of exe file")
        .to_path_buf()
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
