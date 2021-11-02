use crate::entity::error::Error;
use crate::entity::request::{CancelUploadRequest, UploadRequest};
use crate::service::app_state::AppState;
use crate::service::auth::AuthUser;
use crate::util;
use anyhow::Result as AnyResult;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::{Route, State};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn route() -> Vec<Route> {
    routes![pre_upload, upload_file_slices, finish_upload, cancel_upload]
}

// TODO: breakpoint continue
#[post("/pre-upload", data = "<req_body>")]
async fn pre_upload(
    state: &State<AppState>,
    req_body: Json<UploadRequest>,
    _user: AuthUser,
) -> Result<(), Error> {
    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(&storage).join(&util::parse_encoded_url(&req_body.target)?);
    if !target_path.exists() || !target_path.is_dir() {
        return Err(Error::BadRequest);
    }

    let available_space = util::file_system::get_available_space(&storage);
    if available_space > 0 && available_space < req_body.size {
        return Err(Error::BadRequest);
    }

    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&req_body.hash);
    fs::create_dir_all(temp_upload_dir).await?;

    Ok(())
}

#[post("/upload/<hash>/<index>", data = "<file>")]
async fn upload_file_slices(
    hash: &str,
    index: u64,
    mut file: TempFile<'_>,
    _user: AuthUser,
) -> Result<(), Error> {
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(hash);
    if !temp_upload_dir.exists() || !temp_upload_dir.is_dir() {
        return Err(Error::BadRequest);
    }

    let temp_file = temp_upload_dir.join(index.to_string());
    if let Err(e) = file.copy_to(temp_file).await {
        eprintln!("File slice copy error: {:?}", e);
        return Err(Error::InternalServerError);
    }

    Ok(())
}

#[post("/finish-upload", data = "<req_body>")]
async fn finish_upload(
    state: &State<AppState>,
    req_body: Json<UploadRequest>,
    _user: AuthUser,
) -> Result<(), Error> {
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&req_body.hash);
    if !temp_upload_dir.exists() || !temp_upload_dir.is_dir() {
        return Err(Error::BadRequest);
    }

    let storage = state.get_site()?.storage.clone();
    let dir_name = util::parse_encoded_url(&req_body.target)?;
    let target_dir = PathBuf::from(storage).join(dir_name);
    if !target_dir.exists() || target_dir.is_file() {
        return Err(Error::BadRequest);
    }

    let target_file_path = target_dir.join(&req_body.filename);
    if target_file_path.exists() {
        fs::remove_file(&target_file_path).await?;
    }

    if let Err(e) = combine_file_slices(&target_file_path, &temp_upload_dir, req_body.size).await {
        eprintln!("Error when combining file slices: {}", e);
        fs::remove_file(&target_file_path).await?;
        return Err(Error::InternalServerError);
    }

    fs::remove_dir_all(temp_upload_dir).await?;

    Ok(())
}

#[delete("/upload", data = "<req_body>")]
async fn cancel_upload(req_body: Json<CancelUploadRequest>, _user: AuthUser) -> Result<(), Error> {
    for i in 0..req_body.hashes.len() {
        remove_temp_files(&req_body.hashes[i]).await?;
    }

    Ok(())
}

async fn remove_temp_files(hash: &str) -> AnyResult<()> {
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(hash);
    if temp_upload_dir.exists() && temp_upload_dir.is_dir() {
        fs::remove_dir_all(temp_upload_dir).await?;
    }

    Ok(())
}

async fn combine_file_slices(
    target_file_path: &PathBuf,
    temp_upload_dir: &PathBuf,
    req_file_size: u64,
) -> AnyResult<()> {
    let mut target_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&target_file_path)
        .await?;

    let mut index = 0;
    let mut file_slice = temp_upload_dir.join(index.to_string());

    while file_slice.exists() {
        let mut buffer = vec![];
        let mut source_file = File::open(&file_slice).await?;
        source_file.read_to_end(&mut buffer).await?;
        target_file.write_all(&buffer).await?;

        index += 1;
        file_slice = temp_upload_dir.join(index.to_string());
    }

    let filesize = fs::metadata(&target_file_path).await?.len();
    if filesize != req_file_size {
        return Err(anyhow::anyhow!("File size not match"));
    }

    Ok(())
}
