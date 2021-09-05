use crate::util::file_system;
use anyhow::Result as AnyResult;
use regex::Regex;
use std::{os::unix::prelude::MetadataExt, path::PathBuf};
const LIMIT: u64 = 1 * 1024 * 1024;

pub async fn get_track(vtt_path: PathBuf) -> AnyResult<String> {
    if vtt_path.exists() && vtt_path.is_file() {
        if vtt_path.metadata()?.size() > LIMIT {
            return Err(anyhow::anyhow!("Track file too big"));
        }

        return file_system::read_text_file(vtt_path).await;
    }

    let srt_path = vtt_path.with_extension("srt");
    if srt_path.exists() && srt_path.is_file() {
        if srt_path.metadata()?.size() > LIMIT {
            return Err(anyhow::anyhow!("Track file too big"));
        }

        let srt_string = file_system::read_text_file(srt_path).await?;
        return srt_to_vtt(&srt_string).await;
    }

    Err(anyhow::anyhow!("Cannot find track file"))
}

async fn srt_to_vtt(srt_str: &str) -> AnyResult<String> {
    let lines = srt_str.lines();

    let mut vtt_str = String::from("WEBVTT");
    vtt_str.push('\n');
    vtt_str.push('\n');

    let regex_digit = Regex::new(r"^\d+$")?;
    let regex_time = Regex::new(r"^.*-->.*$")?;

    for line in lines {
        if regex_digit.is_match(line) {
            continue;
        }

        if regex_time.is_match(line) {
            let time_line = line.replace(",", ".");
            vtt_str.push_str(&time_line);
            vtt_str.push('\n');
        } else {
            vtt_str.push_str(line);
            vtt_str.push('\n');
        }
    }

    Ok(vtt_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srt_to_vtt() {
        let srt = r#"
1
00:00:06,708 --> 00:00:10,542
我是永尾 我是永尾完治

2
00:00:11,746 --> 00:00:15,580
我现在已经到达羽田了

3
00:00:17,752 --> 00:00:20,687
请问要来接我的人呢

4
00:00:20,755 --> 00:00:24,691
在入境的出口 女性

5
00:00:24,759 --> 00:00:28,593
深蓝色的外套 是
        "#;
        let rt = rocket::tokio::runtime::Runtime::new().unwrap();
        let vtt = rt.block_on(srt_to_vtt(srt)).unwrap();

        println!("Vtt result:\n {}", &vtt);
        assert!(vtt.len() > 0);
    }
}
