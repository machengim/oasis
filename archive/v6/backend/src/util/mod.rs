pub mod db;
pub mod filesystem;
use rand::{distributions::Alphanumeric, Rng};
use std::path::PathBuf;
const SECRET_LENGTH: usize = 32;

pub fn get_config_dir() -> PathBuf {
    match dirs::home_dir() {
        // IMPORTANT! This directory should be changed to config dir later!
        Some(d) => d.join("oasis"),
        None => std::env::current_dir()
            .expect("Cannot get the config directory or the current working directory"),
    }
}

pub fn get_react_dir() -> String {
    match std::env::var("REACT_DIR") {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Cannot get react directory: {}", e),
    }
}

pub fn generate_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LENGTH)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_key() {
        let key = generate_secret_key();
        println!("Get key: {}", &key);
        assert_eq!(key.len(), SECRET_LENGTH);
    }
}
