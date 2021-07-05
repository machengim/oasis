use actix_files::{NamedFile, self};
use actix_web::{get, web, HttpResponse, Result};
use serde::Serialize;
use crate::{entity, filesystem, utils};

#[derive(Serialize, Clone)]
pub struct Response<T> {
    pub data: Option<T>,
    pub error: Option<Error>,
}

#[derive(Serialize, Clone)]
pub struct Error {
    pub code: i16,
    pub message: String,
}

impl<T> Response<T> {
    fn new() -> Self {
        Response {
            data: None,
            error: None,
        }
    }

    fn from_anyhow_result(input: anyhow::Result<T>) -> Self {
        let mut response = Response::new();

        match input {
            Ok(v) => response.data = Some(v),
            Err(e) => {
                debug!("Get error: {}", e);
                response.error = Some(Error {
                    code: 500,
                    message: "Internal server error".to_string(),
                })
            }
        }

        response
    }

    fn custom_err(code: i16, message: &str) -> Self {
        let error = Error{code, message: message.to_owned()};
        Response{data: None, error: Some(error)}
    }
}

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
pub async fn get_volumes(state: web::Data<entity::AppState>) -> HttpResponse {
    let first_run = state.first_run.lock().unwrap();
    if !*first_run {
        let response: Response<String> = Response::custom_err(400, "BAD REQUEST");
        return HttpResponse::Ok().json(response)
    }

    let volumes = filesystem::get_system_volumes();
    let response = Response::from_anyhow_result(volumes);
    HttpResponse::Ok().json(response)   
}
