use crate::{entity::AppState, error::CustomError, filesystem, utils};
use actix_files::{self, NamedFile};
use actix_web::{
    dev::Path,
    get,
    web::{self, Data},
    Either, HttpResponse, Responder, Result,
};
use std::path::PathBuf;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_system_volumes)
            .service(get_system_dirs),
    );
}

pub async fn index(state: Data<AppState>) -> impl Responder {
    let first_run = state.first_run.lock().unwrap();

    if *first_run {
        Either::Left(
            HttpResponse::TemporaryRedirect()
                .header("Location", "/setup")
                .finish(),
        )
    } else {
        match NamedFile::open(utils::build_react_path("index.html")) {
            Ok(nf) => Either::Right(Ok(nf)),
            Err(_) => Either::Right(Err(CustomError::InternalError)),
        }
    }
}

#[get("/fs/volumes")]
async fn get_system_volumes(state: Data<AppState>) -> Result<HttpResponse, CustomError> {
    let first_run = state.first_run.lock().unwrap();
    if !*first_run {
        return Err(CustomError::AuthError);
    }

    match filesystem::get_system_volumes() {
        Ok(volumes) => Ok(HttpResponse::Ok().json(volumes)),
        Err(_) => Err(CustomError::SystemNotSupport),
    }
}

#[get("fs/subdirs/{dir:.*}")]
async fn get_system_dirs(
    dir: web::Path<String>,
    state: Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    let first_run = state.first_run.lock().unwrap();
    if !*first_run {
        return Err(CustomError::AuthError);
    }

    let dir = dir.into_inner();
    println!("dir: {}", &dir);
    let path = PathBuf::from(dir);
    println!("path got: {:?}", &path);
    if !path.is_dir() {
        return Err(CustomError::NotFound);
    }

    match filesystem::get_system_dirs(path).await {
        Ok(dirs) => Ok(HttpResponse::Ok().json(dirs)),
        Err(_) => Err(CustomError::InternalError),
    }
}
