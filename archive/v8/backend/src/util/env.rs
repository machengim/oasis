use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

pub fn must_get_env_value<T: std::str::FromStr>(name: &str, default: T) -> T {
    if let Ok(s) = std::env::var(name) {
        if let Ok(v) = s.parse::<T>() {
            return v;
        }
    }

    default
}

pub fn try_get_env_value<T: std::str::FromStr>(name: &str) -> Result<T> {
    if let Ok(s) = std::env::var(name) {
        if let Ok(v) = s.parse::<T>() {
            return Ok(v);
        }
    }

    Err(anyhow!("Cannot retrieve {} value from .env", name))
}

pub fn get_front_dir() -> anyhow::Result<PathBuf> {
    let path_env: String = try_get_env_value("FRONT_DIR")?;
    let path = Path::new(&path_env);

    Ok(path.to_path_buf())
}

pub fn get_front_index() -> anyhow::Result<PathBuf> {
    let front_dir = get_front_dir()?;
    let path = Path::new(&front_dir).join("index.html");

    Ok(path.to_path_buf())
}
