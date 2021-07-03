use crate::filesystem;
use serde::Serialize;
use warp::{reply::Reply, Filter};

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
}

pub fn get_system_volumes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone
{
    let result = filesystem::get_system_volumes();
    let response = Response::from_anyhow_result(result);

    warp::path!("fs" / "volumes").map(move || warp::reply::json(&response))
}
