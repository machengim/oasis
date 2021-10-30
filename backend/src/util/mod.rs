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
    let path = get_frontend_dir();

    if !path.exists() || !path.is_dir() {
        panic!("Invalid frontend directory");
    }

    path
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

pub fn get_listen_ip_and_port() -> Result<Option<(String, u16)>, anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    let ip;
    let port;
    if args.len() == 1 {
        ip = "0.0.0.0";
        port = 8000;
    } else if args.len() == 3 {
        ip = &args[1];
        match args[2].parse() {
            Ok(p) => {
                port = p;
            }
            _ => return Err(anyhow::anyhow!("Invalid port number")),
        }
    } else {
        return Ok(None);
    }
    Ok(Some((ip.to_string(), port)))
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
