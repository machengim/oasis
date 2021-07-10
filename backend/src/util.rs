use std::path::PathBuf;

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
