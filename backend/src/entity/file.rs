use anyhow::Result as AnyResult;
use rocket::serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct File {
    pub dir: Option<PathBuf>,
    pub filename: String,
    pub file_type: FileType,
    pub size: u64,
}

#[derive(Serialize, PartialEq)]
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
        match ext.to_lowercase().as_str() {
            "c" | "cpp" | "js" | "ts" | "rs" | "py" | "java" | "html" | "css" | "sh" => Self::Code,
            "jpg" | "jpeg" | "gif" | "png" => Self::Image,
            "mp3" | "flac" | "aac" | "ogg" | "wav" => Self::Music,
            "pdf" => Self::Pdf,
            "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" | "wmv" => Self::Video,
            "txt" | "md" | "srt" | "vtt" | "json" | "yml" | "ini" | "conf" => Self::Text,
            _ => Self::Unknown,
        }
    }

    pub fn get_file_type(path: &PathBuf) -> Self {
        match (path.is_dir(), path.extension()) {
            (true, _) => Self::Dir,
            (false, Some(ext)) => Self::infer_file_type(ext.to_str().unwrap_or("")),
            (false, None) => Self::Unknown,
        }
    }
}

impl File {
    pub fn from_path(path: &PathBuf, need_dir: bool, storage: &str) -> AnyResult<Self> {
        let filename = match path.file_name() {
            Some(str) => str.to_string_lossy().to_string(),
            None => {
                return Err(anyhow::anyhow!("Cannot get filename"));
            }
        };

        let file_type = FileType::get_file_type(path);
        let size = match (path.is_dir(), path.metadata()) {
            (false, Ok(meta)) => meta.len(),
            _ => 0,
        };

        let dir = if need_dir {
            let parent_dir = path.parent().unwrap().strip_prefix(storage)?;
            Some(PathBuf::from(parent_dir))
        } else {
            None
        };

        Ok(Self {
            dir,
            filename,
            file_type,
            size,
        })
    }
}
