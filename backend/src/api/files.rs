use crate::entity::error::Error;
use crate::entity::file::{File, FileType};
use crate::entity::request::{CreateDirRequest, GenerateLinkRequest};
use crate::entity::response::FileResponse;
use crate::service::app_state::AppState;
use crate::service::auth::{AuthAdmin, AuthUser};
use crate::service::range::{Range, RangedFile};
use crate::service::track;
use crate::util::{self, file_system};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::{Route, State};
use std::path::PathBuf;
use anyhow::Result as AnyResult;

pub fn route() -> Vec<Route> {
    routes![
        dir_content,
        create_dir,
        file_content,
        video_track,
        generate_share_link,
        get_share_link
    ]
}

#[get("/dir?<path>")]
async fn dir_content(
    path: Option<&str>,
    _user: AuthUser,
    state: &State<AppState>,
) -> Result<Json<Vec<File>>, Error> {
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

#[post("/dir", data = "<req_body>")]
async fn create_dir(
    state: &State<AppState>,
    req_body: Json<CreateDirRequest>,
    _user: AuthAdmin,
) -> Result<(), Error> {
    let storage = state.get_site()?.storage.clone();
    let parent = util::parse_encoded_url(&req_body.parent)?;
    let parent_path = PathBuf::from(storage).join(&parent);
    let target_path = parent_path.join(&req_body.name);

    if !parent_path.exists() || target_path.exists() {
        return Err(Error::BadRequest);
    }

    Ok(fs::create_dir(target_path).await?)
}

#[get("/file/<path>")]
async fn file_content(
    path: &str,
    _user: AuthUser,
    state: &State<AppState>,
    range_header: Range,
) -> Result<FileResponse, Error> {
    let target_path = get_target_path(state, path).map_err(|e| {
        eprintln!("{}", e);
        return 400;
    })?;

    let is_text = FileType::get_file_type(&target_path) == FileType::Text;

    match (range_header.range, is_text) {
        (Some(range), _) => {
            let ranged_file = RangedFile::new(range, target_path).await?;
            Ok(FileResponse::Range(ranged_file))
        },
        (None, true) => Ok(FileResponse::Text(file_system::read_text_file(target_path).await?)),
        (None, false) => Ok(FileResponse::Binary(NamedFile::open(target_path).await?)),
    }
}

// Temporary solution for track file searching.
// Could remove this function in the future.
#[get("/file/track/<path>")]
async fn video_track(
    path: &str,
    _user: AuthUser,
    state: &State<AppState>,
) -> Result<String, Error> {
    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(storage).join(&util::parse_encoded_url(path)?);

    let track_str = match track::get_track(target_path).await {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Error when getting track: {}", e);
            return Err(Error::NotFound);
        }
    };

    Ok(track_str)
}

#[post("/file/share", data = "<req_body>")]
async fn generate_share_link(
    state: &State<AppState>,
    req_body: Json<GenerateLinkRequest>,
    _user: AuthUser,
) -> Result<String, Error> {
    let secret = state.get_secret()?;
    let path_encode = urlencoding::encode(&req_body.path);
    let input = format!("expire={}&path={}", req_body.expire, path_encode);
    let hash = util::sha256(&input, &secret);

    Ok(format!("hash={}&{}", hash, input))
}

#[get("/file/share?<path>&<expire>&<hash>")]
async fn get_share_link(
    state: &State<AppState>,
    path: &str,
    expire: i64,
    hash: &str,
    range_header: Range,
) -> Result<FileResponse, Error> {
    let secret = state.get_secret()?;
    let path_encode = urlencoding::encode(path);

    let input = format!("expire={}&path={}", expire, path_encode);
    if hash != util::sha256(&input, &secret) {
        return Err(Error::BadRequest);
    }

    let target_path = get_target_path(state, path).map_err(|e| {
        eprintln!("{}", e);
        return 400;
    })?;

    // Ok(file_content(path, user, state, range_header).await?)
    match range_header.range {
        Some(range) => {
            let ranged_file = RangedFile::new(range, target_path).await?;
            Ok(FileResponse::Range(ranged_file))
        },
        None => Ok(FileResponse::Binary(NamedFile::open(target_path).await?)),
    }
}


fn get_target_path(state: &State<AppState>, path: &str) -> AnyResult<PathBuf> {
    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(&storage).join(&util::parse_encoded_url(path)?);

    if !target_path.exists() || !target_path.is_file() {
        return Err(anyhow::anyhow!("Invalid file path: {:?}", target_path));
    }

    Ok(target_path)
}