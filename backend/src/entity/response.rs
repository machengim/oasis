use crate::service::range::RangedFile;
use crate::util::constants::{DEFAULT_APP_NAME, DEFAULT_LANGUAGE, DEFAULT_UPDATE_FREQ, VERSION};
use rocket::fs::NamedFile;
use rocket::serde::Serialize;

use super::site::Site;

#[derive(Responder)]
pub enum FileResponse {
    Range(RangedFile),
    Binary(NamedFile),
    Text(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppNeedUpdateResponse {
    pub need: bool,
    pub url: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub username: String,
    pub permission: i8,
    pub expire: usize,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SiteBriefResponse {
    pub name: String,
    pub version: String,
    pub language: String,
    pub update_freq: String,
    pub allow_guest: bool,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SiteFullResponse {
    pub name: String,
    pub version: String,
    pub language: String,
    pub update_freq: String,
    pub storage: String,
    pub allow_guest: bool,
}

impl From<Site> for SiteBriefResponse {
    fn from(s: Site) -> Self {
        Self {
            name: s.name,
            version: s.version,
            language: s.language,
            update_freq: s.update_freq,
            allow_guest: s.allow_guest > 0,
        }
    }
}

impl Default for SiteBriefResponse {
    fn default() -> Self {
        Self {
            name: DEFAULT_APP_NAME.to_owned(),
            version: VERSION.to_owned(),
            language: DEFAULT_LANGUAGE.to_owned(),
            update_freq: DEFAULT_UPDATE_FREQ.to_owned(),
            allow_guest: false,
        }
    }
}

impl From<Site> for SiteFullResponse {
    fn from(s: Site) -> Self {
        Self {
            name: s.name,
            version: s.version,
            language: s.language,
            storage: s.storage,
            update_freq: s.update_freq,
            allow_guest: s.allow_guest > 0,
        }
    }
}

impl Default for SiteFullResponse {
    fn default() -> Self {
        Self {
            name: DEFAULT_APP_NAME.to_owned(),
            version: VERSION.to_owned(),
            language: DEFAULT_LANGUAGE.to_owned(),
            storage: String::new(),
            update_freq: DEFAULT_UPDATE_FREQ.to_owned(),
            allow_guest: false,
        }
    }
}
