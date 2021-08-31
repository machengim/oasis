use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum FileType {
    Dir,
    Code,
    Text,
    Image,
    Music,
    Video,
    Pdf,
    Unknown,
}

impl FileType {
    fn infer_file_type(ext: &str) -> Self {
        match ext {
            "c" | "cpp" | "js" | "ts" | "rs" | "py" | "java" | "html" | "css" => Self::Code,
            "jpg" | "jpeg" | "gif" | "png" => Self::Image,
            "mp3" | "flac" | "aac" | "ogg" => Self::Music,
            "pdf" => Self::Pdf,
            "mp4" | "mov" | "avi" | "mkv" => Self::Video,
            "txt" => Self::Text,
            _ => Self::Unknown,
        }
    }

    pub fn get_file_type(path: &PathBuf) -> Self {
        match (path.is_dir(), path.extension()) {
            (true, _) => Self::Dir,
            (false, Some(ext)) => Self::infer_file_type(ext.to_str().unwrap()),
            (false, None) => Self::Unknown,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct File {
    pub filename: String,
    pub file_type: FileType,
    pub size: u64,
}

impl File {
    pub fn from_path(path: &PathBuf) -> AnyResult<Self> {
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let file_type = FileType::get_file_type(path);
        let meta = path.metadata()?;

        Ok(Self {
            filename,
            file_type,
            size: meta.len(),
        })
    }
}
