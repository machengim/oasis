use std::path::PathBuf;
use std::process::Command;

pub fn get_system_volumes() -> anyhow::Result<Vec<String>> {
    match std::env::consts::OS {
        "linux" => get_linux_volumes(),
        // "macos" => Vec::new(),
        // "windows" => Vec::new(),
        _ => Ok(Vec::new()),
    }
}

fn get_linux_volumes() -> anyhow::Result<Vec<String>> {
    let lsblk_output = Command::new("sh").arg("-c").arg("lsblk").output()?;
    let lines = std::str::from_utf8(&lsblk_output.stdout)?.lines();
    let mut mountpoints: Vec<String> = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let last_word = words.last().unwrap_or(&"");
        if last_word.starts_with('/') {
            mountpoints.push(last_word.to_string());
        }
    }

    Ok(mountpoints)
}

pub async fn get_sub_directories(dir: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Err(anyhow::anyhow!("Not a directory!"));
    }

    let mut dir_iterator = tokio::fs::read_dir(&dir).await?;
    let mut sub_dirs: Vec<PathBuf> = Vec::new();
    while let Some(entry) = dir_iterator.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            sub_dirs.push(path);
        }
    }

    Ok(sub_dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_linux_volumes() {
        let mountpoints = get_linux_volumes().unwrap();
        println!("mountpoints: {:?}", &mountpoints);
        assert!(mountpoints.len() > 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_sub_directories() {
        let path = std::path::Path::new("/");
        let rt = tokio::runtime::Runtime::new().unwrap();
        let sub_directories = rt
            .block_on(get_sub_directories(path.to_path_buf()))
            .unwrap();
        println!("sub_directories: {:?}", &sub_directories);
        assert!(sub_directories.len() > 0);
    }
}
