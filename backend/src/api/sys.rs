use crate::entity::site::{Site, SiteBriefResponse, SiteFullResponse};
use crate::entity::user::User;
use crate::service::app_state::AppState;
use crate::service::error::Error;
use crate::service::token::Token;
use crate::util::{self, file_system};
use rocket::serde::{json::Json, Deserialize};
use rocket::{Either, Route, State};
use sqlx::Connection;
use std::path::PathBuf;

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

pub fn route() -> Vec<Route> {
    routes![system_dirs, sys_volumes, setup, config, update_site]
}

#[post("/sys/setup", data = "<req_body>")]
async fn setup(state: &State<AppState>, req_body: Json<SetupRequest>) -> Result<(), Error> {
    if !state.get_first_run() {
        return Err(Error::Forbidden);
    }

    let storage = util::parse_encoded_url(&req_body.storage)?;

    if req_body.sitename.len() == 0
        || req_body.username.len() < 2
        || req_body.password.len() < 6
        || !storage.exists()
        || !storage.is_dir()
    {
        return Err(Error::BadRequest);
    }

    let current_timestamp = chrono::Utc::now().timestamp_millis();
    let mut conn = state.get_pool_conn().await?;
    let mut tx = conn.begin().await?;

    User::from_setup_req(&req_body, current_timestamp)
        .insert_query(&mut tx)
        .await?;

    Site::new(
        &req_body.sitename,
        &storage,
        &req_body.language,
        current_timestamp,
    )
    .insert(&mut tx)
    .await?;

    tx.commit().await?;

    let new_site = Site::read(&mut conn).await?.ok_or(500)?;
    state.set_first_run(false);
    state.set_site(new_site)?;

    Ok(())
}

#[get("/sys/volumes")]
fn sys_volumes(state: &State<AppState>, token: Token) -> Result<Json<Vec<String>>, Error> {
    if !state.get_first_run() && token.permission != 9 {
        return Err(Error::Unauthorized);
    }

    let volumes = file_system::get_system_volumes()?;

    Ok(Json(volumes))
}

#[get("/sys/dirs/<dir_str>")]
async fn system_dirs(
    state: &State<AppState>,
    token: Token,
    dir_str: &str,
) -> Result<Json<Vec<PathBuf>>, Error> {
    if !state.get_first_run() && token.permission != 9 {
        return Err(Error::Unauthorized);
    }

    let dir = util::parse_encoded_url(dir_str)?;
    let sub_dirs = file_system::get_sub_dirs(&dir).await?;

    Ok(Json(sub_dirs))
}

#[get("/sys/config?<mode>")]
async fn config(
    state: &State<AppState>,
    mode: String,
    token: Token,
) -> Result<Either<Json<SiteBriefResponse>, Json<SiteFullResponse>>, Error> {
    if mode != "brief" && mode != "full" {
        return Err(Error::BadRequest);
    }

    let mut conn = state.get_pool_conn().await?;
    let site_op = Site::read(&mut conn).await?;

    match (mode.as_str(), token.permission, site_op.is_some()) {
        ("brief", _, true) => Ok(Either::Left(Json(SiteBriefResponse::from(
            site_op.unwrap(),
        )))),
        ("brief", _, false) => Ok(Either::Left(Json(SiteBriefResponse::default()))),
        ("full", 9, true) => Ok(Either::Right(Json(SiteFullResponse::from(
            site_op.unwrap(),
        )))),
        ("full", 9, false) => Ok(Either::Right(Json(SiteFullResponse::default()))),
        (_, _, _) => Err(Error::Unauthorized),
    }
}

#[put("/sys/config", data = "<req_body>")]
async fn update_site(
    state: &State<AppState>,
    token: Token,
    req_body: Json<UpdateSiteRequest>,
) -> Result<(), Error> {
    if token.permission != 9 {
        return Err(Error::Unauthorized);
    }

    let storage = util::parse_encoded_url(&req_body.storage)?;
    if !storage.exists() || !storage.is_dir() {
        return Err(Error::BadRequest);
    }

    let storage_str = storage.to_str().unwrap().to_owned();

    let mut conn = state.get_pool_conn().await?;
    let mut site = Site::read(&mut conn).await?.unwrap();
    site.name = req_body.sitename.to_owned();
    site.language = req_body.language.to_owned();
    site.update_freq = req_body.update_freq.to_owned();
    site.storage = storage_str;

    let mut tx = conn.begin().await?;
    site.update(&mut tx).await?;
    tx.commit().await?;

    state.set_site(site)?;

    Ok(())
}
