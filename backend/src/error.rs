use actix_web::{error, http::header, http::StatusCode, HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadRequest,

    #[display(fmt = "system not support")]
    SystemNotSupport,

    #[display(fmt = "not authorized")]
    AuthError,

    #[display(fmt = "resource not found")]
    NotFound,
}

impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::BadRequest => StatusCode::BAD_REQUEST,
            CustomError::SystemNotSupport => StatusCode::BAD_REQUEST,
            CustomError::AuthError => StatusCode::UNAUTHORIZED,
            CustomError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
