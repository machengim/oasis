use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GenerateLinkRequest {
    pub path: String,
    pub expire: i64,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SetupRequest {
    pub sitename: String,
    pub username: String,
    pub password: String,
    pub storage: String,
    pub language: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdateSiteRequest {
    pub sitename: String,
    pub storage: String,
    pub language: String,
    pub update_freq: String,
    pub allow_guest: bool,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangePasswordRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UploadRequest {
    pub filename: String,
    pub size: u64,
    pub target: String,
    pub hash: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UploadSliceRequest {
    pub hash: String,
    pub index: u64,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CancelUploadRequest {
    pub uuids: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateDirRequest {
    pub parent: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RenameFileRequest {
    pub new_name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SetFileVisibilityRequest {
    pub visible: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ForgotPasswordRequest {
    pub url: String,
    pub username: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResetPasswordRequest {
    pub uuid: String,
    pub code: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CopyMoveFileRequest {
    pub source: String,
    pub target: String,
}
