use crate::entity::error::Error;
use crate::entity::request::{UploadRequest, UploadSliceRequest};
use crate::service::app_state::AppState;
use crate::service::auth::AuthUser;
use crate::util;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::{Route, State};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn route() -> Vec<Route> {
    routes![pre_upload, upload_file_slices, finish_upload]
}

// TODO: check available disk space before uploading
// TODO: breakpoint continue
#[post("/pre-upload", data = "<req_body>")]
async fn pre_upload(
    state: &State<AppState>,
    req_body: Json<UploadRequest>,
    _user: AuthUser,
) -> Result<(), Error> {
    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(storage).join(&util::parse_encoded_url(&req_body.target)?);
    if !target_path.exists() || !target_path.is_dir() {
        return Err(Error::BadRequest);
    }

    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&req_body.hash);
    fs::create_dir_all(temp_upload_dir).await?;

    Ok(())
}

#[post("/upload", data = "<req_body>")]
async fn upload_file_slices(
    req_body: Json<UploadSliceRequest>,
    _user: AuthUser,
) -> Result<(), Error> {
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&req_body.hash);
    if !temp_upload_dir.exists() || !temp_upload_dir.is_dir() {
        return Err(Error::BadRequest);
    }

    let temp_file = temp_upload_dir.join(&req_body.index.to_string());
    let mut file = File::create(temp_file).await?;
    file.write_all(&req_body.data).await?;

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
    if filesize != req_body.size {
        return Err(Error::BadRequest);
    }

    // TODO: check file md5 before complete.

    fs::remove_dir_all(temp_upload_dir).await?;

    Ok(())
}

// TODO: cancel upload by user
