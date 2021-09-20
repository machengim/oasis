use crate::entity::site::{Site, SiteBasic};
use crate::entity::user::User;
use crate::service::app_state::AppState;
use crate::service::error::Error;
use crate::util::{self, file_system};
use rocket::serde::{json::Json, Deserialize};
use rocket::{Route, State};
use sqlx::Connection;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
    pub language: String,
}

pub fn route() -> Vec<Route> {
    routes![system_dirs, sys_volumes, setup, site_basic]
}

#[post("/sys/setup", data = "<req_body>")]
async fn setup(state: &State<AppState>, req_body: Json<SetupRequest>) -> Result<(), Error> {
    if !state.get_first_run() {
        return Err(Error::Forbidden);
    }

    let storage = util::parse_encoded_url(&req_body.storage)?;

    if req_body.username.len() < 2
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

    Site::new(&storage, &req_body.language, current_timestamp)
        .insert_query(&mut tx)
        .await?;

    tx.commit().await?;

    let new_site = Site::read(&mut conn).await?.ok_or(500)?;
    state.set_first_run(false);
    state.set_site(new_site)?;

    Ok(())
}

#[get("/sys/volumes")]
fn sys_volumes(state: &State<AppState>) -> Result<Json<Vec<String>>, Error> {
    if !state.get_first_run() {
        return Err(Error::Unauthorized);
    }

    let volumes = file_system::get_system_volumes()?;

    Ok(Json(volumes))
}

#[get("/sys/dirs/<dir_str>")]
async fn system_dirs(state: &State<AppState>, dir_str: &str) -> Result<Json<Vec<PathBuf>>, Error> {
    if !state.get_first_run() {
        return Err(Error::Forbidden);
    }

    let dir = util::parse_encoded_url(dir_str)?;
    let sub_dirs = file_system::get_sub_dirs(&dir).await?;

    Ok(Json(sub_dirs))
}

#[get("/site/basic")]
async fn site_basic(state: &State<AppState>) -> Result<Json<SiteBasic>, Error> {
    let site = state.get_site()?;
    let site_basic = SiteBasic::new(&site.language, &site.version);

    Ok(Json(site_basic))
}
