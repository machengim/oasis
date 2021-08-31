use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Conflict,
    Forbidden,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadRequest => f.write_str("BadRequest"),
            Error::Conflict => f.write_str("Conflict"),
            Error::Forbidden => f.write_str("Forbidden"),
            Error::InternalServerError => f.write_str("InternalServerError"),
            Error::NotFound => f.write_str("NotFound"),
            Error::Unauthorized => f.write_str("Unauthorized"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadRequest => "BadRequest",
            Error::Conflict => "Conflict",
            Error::Forbidden => "Forbidden",
            Error::InternalServerError => "InternalServerError",
            Error::NotFound => "NotFound",
            Error::Unauthorized => "Unauthorized",
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Error::BadRequest => Err(Status::BadRequest),
            Error::Conflict => Err(Status::Conflict),
            Error::Forbidden => Err(Status::Forbidden),
            Error::InternalServerError => Err(Status::InternalServerError),
            Error::NotFound => Err(Status::NotFound),
            Error::Unauthorized => Err(Status::Unauthorized),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Error::InternalServerError
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Error::InternalServerError
    }
}

impl From<i32> for Error {
    fn from(number: i32) -> Self {
        match number {
            400 => Error::BadRequest,
            409 => Error::Conflict,
            401 => Error::Unauthorized,
            403 => Error::Forbidden,
            404 => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}
