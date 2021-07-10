use crate::entity::Auth;
use crate::filesystem;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use std::path::PathBuf;

pub fn serve_api() -> Vec<Route> {
    routes![system_volumes, system_sub_dirs]
}

#[get("/sys/volumes")]
fn system_volumes(_auth: Auth) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_volumes() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::NotImplemented),
    }
}

// TODO: Should prevent reading dirs outside of the mountpoints.
#[get("/sys/dirs/<dir..>")]
async fn system_sub_dirs(dir: PathBuf) -> Result<Json<Vec<String>>, Status> {
    println!(" ============ Got dir {:?} ===============", &dir);
    match filesystem::get_system_dirs(dir).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!("Got error when retrieving sub dirs: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
