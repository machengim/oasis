use crate::entity::file::File;
use crate::service::app_state::AppState;
use crate::service::error::Error;
use crate::service::range::RangedFile;
use crate::service::token::Token;
use crate::service::track;
use crate::util::{self, file_system};
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::{Route, State};
use std::path::PathBuf;

pub fn route() -> Vec<Route> {
    routes![dir_content, file_content, video_track, text_file_content]
}

#[get("/dir?<path>")]
async fn dir_content(
    path: Option<&str>,
    token: Token,
    state: &State<AppState>,
) -> Result<Json<Vec<File>>, Error> {
    if token.uid <= 0 || token.permission <= 0 {
        return Err(Error::Unauthorized);
    }

    let storage = state.get_site()?.storage.clone();
    let target_path = match path {
        Some(dir) => PathBuf::from(storage).join(&util::parse_encoded_url(dir)?),
        None => PathBuf::from(storage),
    };

    if !target_path.exists() || !target_path.is_dir() {
        eprintln!("Invalid dir path: {:?}", &target_path);
        return Err(Error::BadRequest);
    }

    let mut dir_iterator = fs::read_dir(target_path).await?;
    let mut content: Vec<File> = Vec::new();
    while let Some(entry) = dir_iterator.next_entry().await? {
        let path = entry.path();
        content.push(File::from_path(&path)?);
    }

    Ok(Json(content))
}

#[get("/file?<path>")]
async fn file_content(
    path: &str,
    token: Token,
    state: &State<AppState>,
) -> Result<RangedFile, Error> {
    if token.uid <= 0 || token.permission <= 0 {
        return Err(Error::Unauthorized);
    }

    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(&storage).join(&util::parse_encoded_url(path)?);

    if !target_path.exists() || !target_path.is_file() {
        eprintln!("Invalid file path: {:?}", &target_path);
        return Err(Error::BadRequest);
    }

    Ok(RangedFile { path: target_path })
}

#[get("/file/track?<path>")]
async fn video_track(path: &str, token: Token, state: &State<AppState>) -> Result<String, Error> {
    if token.uid <= 0 || token.permission <= 0 {
        return Err(Error::Unauthorized);
    }

    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(&storage).join(&util::parse_encoded_url(path)?);

    let track_str = match track::get_track(target_path).await {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Error when getting track: {}", e);
            return Err(Error::NotFound);
        }
    };

    Ok(track_str)
}

#[get("/file/text?<path>")]
async fn text_file_content(
    path: &str,
    token: Token,
    state: &State<AppState>,
) -> Result<String, Error> {
    if token.uid <= 0 || token.permission <= 0 {
        return Err(Error::Unauthorized);
    }

    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(&storage).join(&util::parse_encoded_url(path)?);

    if !target_path.exists() || !target_path.is_file() {
        eprintln!("Invalid file path: {:?}", &target_path);
        return Err(Error::NotFound);
    }

    Ok(file_system::read_text_file(target_path).await?)
}
