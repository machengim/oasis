use crate::auth::{AppDb, AppFirstRun};
use crate::db;
use crate::entity::{AppState, SetupRequest};
use crate::filesystem;
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use std::path::PathBuf;

pub fn serve_api() -> Vec<Route> {
    routes![
        get_system_volumes_first_run,
        get_system_volumes_default,
        get_system_dirs_first_run,
        get_system_dirs_default,
        post_setup_first_run,
        post_setup_default
    ]
}

#[get("/sys/volumes", rank = 1)]
fn get_system_volumes_first_run(_auth: AppFirstRun) -> Result<Json<Vec<String>>, Status> {
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
    _auth: AppFirstRun,
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

// // TODO: unifinished.
#[post("/setup", rank = 1, data = "<setup_req>")]
async fn post_setup_first_run(
    setup_req: Json<SetupRequest>,
    mut db: AppDb,
    state: &State<AppState>,
) -> Result<(), Status> {
    let encrypt_password = match hash("hunter2", DEFAULT_COST) {
        Ok(v) => v,
        Err(_) => return Err(Status::InternalServerError),
    };

    let sql = "insert into USER (username, password) values (?1, ?2)";
    let args = vec![setup_req.username.clone(), encrypt_password];
    if let Err(e) = db::execute(sql, args, &mut db.conn).await {
        eprintln!("{}", e);
    }

    let mut first_run = state.first_run.lock().unwrap();
    *first_run = false;

    Ok(())
}

#[post("/setup", rank = 2)]
fn post_setup_default() -> Status {
    Status::BadRequest
}
