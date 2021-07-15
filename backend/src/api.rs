use crate::db;
use crate::entity::{AppState, AuthDb, AuthIndex, SetupRequest};
use crate::filesystem;
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use std::path::PathBuf;

pub fn serve_api() -> Vec<Route> {
    routes![system_volumes, system_sub_dirs, setup]
}

#[get("/sys/volumes")]
fn system_volumes(_auth: AuthIndex) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_volumes() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::NotImplemented),
    }
}

#[get("/sys/dirs/<dir..>")]
async fn system_sub_dirs(dir: PathBuf, _auth: AuthIndex) -> Result<Json<Vec<String>>, Status> {
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
async fn setup(
    setup_req: Json<SetupRequest>,
    mut auth: AuthDb,
    state: &State<AppState>,
) -> Result<(), Status> {
    let encrypt_password = match hash("hunter2", DEFAULT_COST) {
        Ok(v) => v,
        Err(_) => return Err(Status::InternalServerError),
    };

    let sql = "insert into USER (username, password) values (?1, ?2)";
    let args = vec![setup_req.username.clone(), encrypt_password];
    if let Err(e) = db::execute(sql, args, &mut auth.conn).await {
        eprintln!("{}", e);
    }

    let mut first_run = state.first_run.lock().unwrap();
    *first_run = false;

    Ok(())
}
