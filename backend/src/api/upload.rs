use crate::entity::error::Error;
use crate::entity::request::{CancelUploadRequest, UploadRequest};
use crate::entity::upload_task::UploadTask;
use crate::service::app_state::AppState;
use crate::service::auth::AuthAdmin;
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
    user: AuthAdmin,
) -> Result<String, Error> {
    let storage = state.get_site()?.storage.clone();
    let target_dir = PathBuf::from(&storage).join(&util::parse_encoded_url(&req_body.target)?);
    if !target_dir.exists() || !target_dir.is_dir() {
        return Err(Error::BadRequest);
    }

    let available_space = util::file_system::get_available_space(&storage);
    if available_space > 0 && available_space < req_body.size {
        return Err(Error::BadRequest);
    }

    let upload_task = UploadTask::new(&req_body, user.uid, target_dir);
    let uuid = upload_task.uuid.clone();
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&upload_task.uuid);
    fs::create_dir_all(temp_upload_dir).await?;
    state.push_upload_task(upload_task)?;

    Ok(uuid)
}

#[post("/upload/<uuid>/<index>", data = "<file>")]
async fn upload_file_slices(
    state: &State<AppState>,
    uuid: &str,
    index: u64,
    mut file: TempFile<'_>,
    user: AuthAdmin,
) -> Result<(), Error> {
    let task = match state.find_upload_uuid(uuid)? {
        Some(v) => v,
        None => return Err(Error::BadRequest),
    };

    if task.userid != user.uid {
        return Err(Error::Unauthorized);
    }

    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(uuid);
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

#[post("/finish-upload/<uuid>")]
async fn finish_upload(state: &State<AppState>, uuid: &str, _user: AuthAdmin) -> Result<(), Error> {
    let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(uuid);
    if !temp_upload_dir.exists() || !temp_upload_dir.is_dir() {
        return Err(Error::BadRequest);
    }

    let task = state.find_upload_uuid(uuid)?.unwrap();
    let target_dir = &task.dir;
    if !target_dir.exists() || target_dir.is_file() {
        return Err(Error::BadRequest);
    }

    let target_file_path = target_dir.join(&task.filename);
    if target_file_path.exists() {
        fs::remove_file(&target_file_path).await?;
    }

    if let Err(e) = combine_file_slices(&target_file_path, &temp_upload_dir, task.size).await {
        eprintln!("Error when combining file slices: {}", e);
        fs::remove_file(&target_file_path).await?;
        return Err(Error::InternalServerError);
    }

    fs::remove_dir_all(&temp_upload_dir).await?;
    state.remove_upload_task(task)?;

    Ok(())
}

#[post("/cancel-upload", data = "<req_body>")]
async fn cancel_upload(
    state: &State<AppState>,
    req_body: Json<CancelUploadRequest>,
    user: AuthAdmin,
) -> Result<(), Error> {
    for uuid in req_body.uuids.iter() {
        remove_upload_task(state, uuid, &user).await?;
    }

    Ok(())
}

async fn remove_upload_task(
    state: &State<AppState>,
    uuid: &str,
    user: &AuthAdmin,
) -> AnyResult<()> {
    if let Some(task) = state.find_upload_uuid(uuid)? {
        if task.userid != user.uid {
            return Err(anyhow::anyhow!("User id not match to remove task"));
        }

        let temp_upload_dir = PathBuf::from(util::get_temp_path()).join(&task.uuid);
        if temp_upload_dir.exists() && temp_upload_dir.is_dir() {
            fs::remove_dir_all(temp_upload_dir).await?;
        }

        state.remove_upload_task(task)?;
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

    let mut index: u64 = 1; // Change to start index from 1.
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
