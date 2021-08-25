use anyhow::Result;
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use tide::http::headers::HeaderValues;
const DEFAULT_RANGE_SIZE: u64 = 1024 * 1024;

#[derive(Debug)]
pub struct Range {
    start: Option<u64>,
    end: Option<u64>,
}

impl Range {
    pub fn from(range_req: &str) -> Self {
        let v: Vec<&str> = range_req.split(|c| c == '=' || c == '-').collect();
        let start: Option<u64> = parse_range_str(v.get(1));
        let end: Option<u64> = parse_range_str(v.get(2));

        Range { start, end }
    }
}

pub fn read_file_meta(path: &PathBuf) -> Result<(File, u64)> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();

    Ok((file, size))
}

fn read_file_bytes(file: &mut File, start: u64, size: usize) -> Result<Vec<u8>> {
    file.seek(SeekFrom::Start(start))?;
    let mut contents = vec![0u8; size];
    file.read_exact(&mut *contents)?;

    Ok(contents)
}

pub fn get_header_contents(
    start: u64,
    end: u64,
    size: u64,
    file: &mut File,
) -> Result<(Vec<u8>, String)> {
    let len = end - start + 1;
    let contents = read_file_bytes(file, start, len as usize)?;
    let content_range = format!("bytes {}-{}/{}", start, end, size);

    Ok((contents, content_range))
}

pub fn get_range_length(size: u64, range_header: Option<&HeaderValues>) -> (u64, u64) {
    match range_header {
        Some(range_strs) => {
            let range_str = range_strs.get(0).unwrap().to_string();
            let range = Range::from(&range_str);
            return get_some_range_length(size, range);
        }
        None => get_none_range_length(size),
    }
}

fn parse_range_str(input: Option<&&str>) -> Option<u64> {
    let str_value = match input {
        Some(v) => v,
        None => return None,
    };

    let number: u64 = match str_value.parse() {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some(number)
}

fn get_none_range_length(size: u64) -> (u64, u64) {
    let start = 0;
    let end = min(start + DEFAULT_RANGE_SIZE, size - 1);

    (start, end)
}

fn get_some_range_length(size: u64, range: Range) -> (u64, u64) {
    let mut start = range.start.unwrap_or(0);
    if start >= size {
        start = size - 1;
    }

    let mut end = match range.end {
        Some(range_end) => range_end,
        None => start + DEFAULT_RANGE_SIZE,
    };

    if end < start {
        end = start;
    } else if end >= size {
        end = size - 1;
    }

    (start, end)
}
