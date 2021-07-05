use actix_files::{NamedFile, self};
use actix_web::{get, web, HttpResponse, Result};
use crate::{entity, filesystem, utils};

#[get("/")]
pub async fn index(state: web::Data<entity::AppState>) -> Result<NamedFile> {
    let first_run = state.first_run.lock().unwrap();

    if *first_run {
        // Ok(HttpResponse::TemporaryRedirect().header("Location", "/setup/index.html").finish())
        Ok(NamedFile::open(utils::build_react_path("setup/index.html"))?)
    } else {
        // HttpResponse::Found().header("Location", "/").finish()
        Ok(NamedFile::open(utils::build_react_path("index.html"))?)
    }
}

#[get("/fs/volumes")]
pub async fn get_volumes() -> Result<HttpResponse> {
    let volumes = filesystem::get_system_volumes()?;
    Ok(HttpResponse::Ok().json(volumes))    
}
