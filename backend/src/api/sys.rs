use crate::entity::auth::FirstRun;
use crate::util::filesystem;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use std::path::PathBuf;

pub fn route() -> Vec<Route> {
    routes![
        get_system_volumes_first_run,
        get_system_volumes_default,
        get_system_dirs_first_run,
        get_system_dirs_default,
    ]
}

#[get("/sys/volumes", rank = 1)]
fn get_system_volumes_first_run(_auth: FirstRun) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_volumes() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::NotImplemented),
    }
}

#[get("/sys/volumes", rank = 2)]
fn get_system_volumes_default() -> Status {
    Status::BadRequest
}

#[get("/sys/dirs/<dir..>", rank = 1)]
async fn get_system_dirs_first_run(
    dir: PathBuf,
    _auth: FirstRun,
) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_dirs(dir).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!("Got error when retrieving sub dirs: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/sys/dirs", rank = 2)]
async fn get_system_dirs_default() -> Status {
    Status::BadRequest
}
