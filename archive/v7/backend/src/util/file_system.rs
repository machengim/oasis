use async_std::fs;
use async_std::path::{Path, PathBuf};
use async_std::prelude::*;
// use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_system_volumes() -> anyhow::Result<Vec<String>> {
    match std::env::consts::OS {
        "linux" => get_linux_volumes(),
        "macos" => get_mac_volumes(),
        // "windows" => Vec::new(),
        _ => Err(anyhow::anyhow!("Not supported system")),
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

fn get_mac_volumes() -> anyhow::Result<Vec<String>> {
    let df_output = Command::new("sh").arg("-c").arg("df -Hl").output()?;
    let lines = std::str::from_utf8(&df_output.stdout)?.lines();
    let mut mountpoints: Vec<String> = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let last_word = words.last().unwrap_or(&"");
        if last_word.starts_with('/') && !last_word.starts_with("/System") {
            mountpoints.push(last_word.to_string());
        }
    }
    Ok(mountpoints)
}

// Due to the complexity of the filename encoding methods on different OSs,
// the conversion between OsString and String should be double checked.
// Besides, the automatic PathBuf conversion from the request uri
// should be tested on different OSs as well.
pub async fn get_system_dirs(dir: &str) -> anyhow::Result<Vec<String>> {
    let dir = PathBuf::from(dir);
    let dir_absolute = match dir.is_absolute() {
        true => dir,
        false => Path::new("/").join(dir),
    };

    if !dir_absolute.is_dir().await {
        return Err(anyhow::anyhow!("Not a directory!"));
    }

    let mut dir_iterator = fs::read_dir(&dir_absolute).await?;
    let mut sub_dirs: Vec<String> = Vec::new();
    while let Some(entry) = dir_iterator.next().await {
        let path = entry?.path();
        if path.is_dir().await {
            match path.into_os_string().into_string() {
                Ok(v) => sub_dirs.push(v.to_owned()),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Cannot convert path name to string: {:?}",
                        e
                    ))
                }
            }
        }
    }

    Ok(sub_dirs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task::block_on;

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
        let path = "/home";
        // let rt = tokio::runtime::Runtime::new().unwrap();
        let sub_directories = block_on(get_system_dirs(path)).unwrap();

        println!("sub_directories: {:?}", &sub_directories);
        assert!(sub_directories.len() > 0);
    }
}
