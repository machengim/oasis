use crate::entity::upload::PreUploadRequest;
use rocket::serde::json::Json;
use rocket::Route;

pub fn route() -> Vec<Route> {
    routes![post_pre_upload]
}

// TODO: auth user
#[post("/pre_upload", data = "<request>")]
async fn post_pre_upload(request: Json<PreUploadRequest>) -> Result<(), std::io::Error> {
    println!("{:?}", request);
    Ok(())
}
