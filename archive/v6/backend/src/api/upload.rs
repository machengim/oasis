use crate::entity::app;
use crate::entity::auth;
use crate::entity::upload::PreUploadRequest;
use crate::entity::upload::{FinishUploadRequest, SliceUploadRequest, Upload, UploadState};
use crate::util::filesystem;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn route() -> Vec<Route> {
    routes![post_pre_upload, post_upload, post_finish_upload]
}

#[post("/pre_upload", data = "<pre_upload_req>")]
async fn post_pre_upload(
    pre_upload_req: Json<PreUploadRequest>,
    user: auth::AuthUser,
    storage: app::Storage,
    upload_state: &State<UploadState>,
) -> Result<String, Status> {
    let upload_id = uuid::Uuid::new_v4().to_string();
    if let Err(e) = filesystem::create_upload_folder(&storage.path, &upload_id).await {
        eprintln!("Get error when creating upload folder: {}", e);
        return Err(Status::InternalServerError);
    }

    let upload = Upload::from(
        pre_upload_req.into_inner(),
        upload_id.clone(),
        user.claim.uid,
    );

    let mut uploads = upload_state.uploads.lock().unwrap();
    (*uploads).push(upload);

    Ok(upload_id)
}

#[post("/upload/<upload_id>", data = "<slice_req>")]
async fn post_upload(
    upload_id: String,
    slice_req: Json<SliceUploadRequest>,
    user: auth::AuthUser,
    storage: app::Storage,
    upload_state: &State<UploadState>,
) -> Result<(), Status> {
    match check_upload_info(upload_state, upload_id.clone(), user.claim.uid) {
        Ok(true) => (),
        Ok(false) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("{}", e);
            return Err(Status::InternalServerError);
        }
    };
    if let Err(e) = slice_req.write_to_temp(&storage.path, &upload_id).await {
        eprintln!("Cannot write slice to tmp folder: {}", e);
        return Err(Status::InternalServerError);
    }
    Ok(())
}

// Tasks: validate upload info, combine files and write to storage folder,
// add record to db, remove tmp folder, return file id.
#[post("/finish-upload/<upload_id>", data = "<finish_req>")]
async fn post_finish_upload(
    upload_id: String,
    finish_req: Json<FinishUploadRequest>,
    user: auth::AuthUser,
    storage: app::Storage,
    upload_state: &State<UploadState>,
) -> Result<(), Status> {
    match check_upload_info(upload_state, upload_id.clone(), user.claim.uid) {
        Ok(true) => (),
        Ok(false) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("{}", e);
            return Err(Status::InternalServerError);
        }
    };

    if let Err(e) = finish_req.into_inner().combine_slices(&storage.path).await {
        eprintln!("{}", e);
        return Err(Status::InternalServerError);
    }
    Ok(())
}

fn check_upload_info(
    upload_state: &State<UploadState>,
    upload_id: String,
    user_id: i64,
) -> anyhow::Result<bool> {
    let uploads = match upload_state.uploads.lock() {
        Ok(state) => state,
        Err(e) => {
            return Err(anyhow::anyhow!("Cannot get upload state: {}", e));
        }
    };
    let find_upload: Vec<&Upload> = (*uploads)
        .iter()
        .filter(|u| u.upload_id == upload_id)
        .collect();
    let find_upload_in_state = match find_upload.get(0) {
        Some(upload) => upload,
        None => return Ok(false),
    };

    if find_upload_in_state.owner != user_id {
        return Ok(false);
    }

    Ok(true)
}
