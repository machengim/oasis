use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::cmp::min;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::PathBuf;

const DEFAULT_RANGE_SIZE: u64 = 1024 * 1024;

pub struct RangedFile {
    pub path: PathBuf,
}

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

impl<'r, 'o: 'r> Responder<'r, 'o> for RangedFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let path = self.path;
        let (mut file, size) = match read_file_meta(&path) {
            Ok((file, size)) => (file, size),
            Err(_) => return Err(Status::InternalServerError),
        };
        let file_type = read_file_ext(&path);

        let range_header = req.headers().get_one("Range");
        let (start, end) = get_range_length(size, range_header);
        let len = end - start + 1;

        let (contents, content_range) = match get_header_contents(start, end, size, &mut file) {
            Ok((v, l)) => (v, l),
            Err(_) => return Err(Status::InternalServerError),
        };

        Response::build()
            .status(Status::PartialContent)
            .raw_header("Content-Type", file_type)
            .raw_header("Content-Range", content_range)
            .raw_header("Content-Length", len.to_string())
            .sized_body(len as usize, Cursor::new(contents))
            .ok()
    }
}

fn get_header_contents(
    start: u64,
    end: u64,
    size: u64,
    file: &mut File,
) -> anyhow::Result<(Vec<u8>, String)> {
    let len = end - start + 1;
    let contents = read_file_bytes(file, start, len as usize)?;
    let content_range = format!("bytes {}-{}/{}", start, end, size);

    Ok((contents, content_range))
}

fn get_range_length(size: u64, range_header: Option<&str>) -> (u64, u64) {
    match range_header {
        Some(range_str) => {
            let range = Range::from(range_str);
            return get_some_range_length(size, range);
        }
        None => get_none_range_length(size),
    }
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

fn read_file_meta(path: &PathBuf) -> anyhow::Result<(File, u64)> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();

    Ok((file, size))
}

// TODO: to avoid reading file extension every time
// consider saving this info inside the database.
fn read_file_ext(path: &PathBuf) -> &'static str {
    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("") {
            "mp4" | "mkv" | "avi" => return "video/mp4",
            "mp3" => return "audio/mpeg",
            "png" => return "image/png",
            "jpg" | "jpeg" => return "image/jpeg",
            "txt" | "css" | "js" => return "text/plain",
            "htm" | "html" => return "text/html",
            _ => (),
        }
    }

    "unknown"
}

fn read_file_bytes(file: &mut File, start: u64, size: usize) -> anyhow::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(start))?;
    let mut contents = vec![0u8; size];
    file.read_exact(&mut *contents)?;

    Ok(contents)
}
