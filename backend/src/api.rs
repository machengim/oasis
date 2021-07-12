use crate::db;
use crate::entity::{Auth, SetupRequest};
use crate::filesystem;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use std::path::PathBuf;

pub fn serve_api() -> Vec<Route> {
    routes![system_volumes, system_sub_dirs, setup]
}

#[get("/sys/volumes")]
fn system_volumes(_auth: Auth) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_volumes() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::NotImplemented),
    }
}

#[get("/sys/dirs/<dir..>")]
async fn system_sub_dirs(dir: PathBuf, _auth: Auth) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_dirs(dir).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!("Got error when retrieving sub dirs: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// TODO: unifinished.
#[post("/setup", data = "<setup_req>")]
async fn setup(setup_req: Json<SetupRequest>, _auth: Auth) -> Result<(), Status> {
    let encrypt_password = match hash("hunter2", DEFAULT_COST) {
        Ok(v) => v,
        Err(e) => return Err(Status::InternalServerError),
    };

    Ok(())
}
