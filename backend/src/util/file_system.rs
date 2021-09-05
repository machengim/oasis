use anyhow::Result as AnyResult;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use rocket::tokio::fs;
use std::path::PathBuf;
use std::process::Command;
use tokio::io::AsyncReadExt;

pub fn get_system_volumes() -> AnyResult<Vec<String>> {
    match std::env::consts::OS {
        "linux" => get_linux_volumes(),
        "macos" => get_mac_volumes(),
        // "windows" => Vec::new(),
        _ => Err(anyhow::anyhow!("Not supported system")),
    }
}

fn get_linux_volumes() -> AnyResult<Vec<String>> {
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

fn get_mac_volumes() -> AnyResult<Vec<String>> {
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

pub async fn get_sub_dirs(dir: &PathBuf) -> AnyResult<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Err(anyhow::anyhow!("Not a directory!"));
    }

    let mut dir_iterator = fs::read_dir(dir).await?;
    let mut sub_dirs: Vec<PathBuf> = Vec::new();
    while let Some(entry) = dir_iterator.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            sub_dirs.push(path);
        }
    }

    Ok(sub_dirs)
}

// All text file needs to be read after checking its encoding method.
pub async fn read_text_file(path: PathBuf) -> AnyResult<String> {
    let mut buffer = vec![];
    let mut file = fs::File::open(path).await?;
    file.read_to_end(&mut buffer).await?;

    let encoding = detect_encoding(&buffer)?;
    let (cow, _encoding, malformed) = encoding.decode(&buffer);
    if malformed {
        return Err(anyhow::anyhow!("File encoding malformed"));
    }

    Ok(cow.to_owned().to_string())
}

fn detect_encoding(buffer: &[u8]) -> AnyResult<&'static Encoding> {
    let mut detector = EncodingDetector::new();
    detector.feed(buffer, true);
    let (encoding, liable) = detector.guess_assess(None, true);
    if !liable {
        return Err(anyhow::anyhow!("Cannot detect encoding method"));
    }

    Ok(encoding)
}

#[cfg(test)]
mod test {
    use super::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

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
        let path = PathBuf::from("/home");
        let rt = Runtime::new().unwrap();
        let sub_directories = rt.block_on(get_sub_dirs(&path)).unwrap();

        println!("sub_directories: {:?}", &sub_directories);
        assert!(sub_directories.len() > 0);
    }

    #[test]
    fn test_get_file_encoding() {
        let pwd = std::env::current_dir().unwrap();
        let path = pwd.join("resources/tests/01.srt");
        let rt = rocket::tokio::runtime::Runtime::new().unwrap();
        let decoded_str = rt.block_on(read_text_file(path)).unwrap();
        println!("Decoded string: {}", &decoded_str);
        assert!(decoded_str.len() > 0);
    }
}
