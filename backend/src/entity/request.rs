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
