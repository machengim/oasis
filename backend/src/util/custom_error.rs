use tide::{Error, StatusCode};

pub struct CustomError {
    pub code: StatusCode,
    pub info: String,
}

impl CustomError {
    pub fn from<T: Into<String>>(code: StatusCode, info: T) -> Error {
        Error::new(code, anyhow::anyhow!(info.into()))
    }
}
