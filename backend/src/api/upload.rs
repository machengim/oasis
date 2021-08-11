use crate::entity::auth;
use crate::entity::upload::PreUploadRequest;
use crate::entity::upload::{Upload, UploadState};
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn route() -> Vec<Route> {
    routes![post_pre_upload]
}

#[post("/pre_upload", data = "<request>")]
async fn post_pre_upload(
    request: Json<PreUploadRequest>,
    user: auth::AuthUser,
    state: &State<UploadState>,
) -> Result<String, std::io::Error> {
    let upload = Upload::from(request.into_inner(), user.claim.uid);
    let upload_id = upload.upload_id.clone();
    let mut uploads = state.uploads.lock().unwrap();
    (*uploads).push(upload);
    println!("current upload list is {:?}", *uploads);

    Ok(upload_id)
}
