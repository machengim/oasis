pub mod env;
pub mod file_system;
pub mod init;
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
