use std::path::PathBuf;

pub fn build_react_path(p: &str) -> PathBuf{
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");
    let mut path = PathBuf::new();
    path.push(react_dir); path.push(p);

    path
}

pub fn get_config_dir() -> PathBuf {
    match dirs::home_dir() {
        // IMPORTANT! This directory should be changed to config dir later!
        Some(d) => d.join("oasis"),
        None => std::env::current_dir()
            .expect("Cannot get the config directory or the current working directory"),
    }
}

pub fn get_version() -> anyhow::Result<f64> {
    let version: f64 = match std::env::var("VERSION")?.parse() {
        Ok(v) => v,
        _ => 0.1,
    };

    Ok(version)
}