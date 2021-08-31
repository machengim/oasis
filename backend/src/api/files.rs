use crate::entity::file::File;
use crate::service::app_state::AppState;
use crate::service::error::Error;
use crate::service::token::Token;
use crate::util;
use rocket::serde::json::Json;
use rocket::tokio::fs;
use rocket::{Route, State};
use std::path::PathBuf;

pub fn route() -> Vec<Route> {
    routes![dir_content]
}

#[get("/dir/<dir>")]
async fn dir_content(
    dir: &str,
    token: Token,
    state: &State<AppState>,
) -> Result<Json<Vec<File>>, Error> {
    if token.uid <= 0 || token.permission <= 0 {
        return Err(Error::Unauthorized);
    }

    let dir = util::parse_encoded_url(dir)?;
    let storage = state.get_site()?.storage.clone();
    let target_path = PathBuf::from(storage).join(&dir);

    if !target_path.exists() || !target_path.is_dir() {
        eprintln!("Invalid path: {:?}", &target_path);
        return Err(Error::BadRequest);
    }

    let mut dir_iterator = fs::read_dir(target_path).await?;
    let mut content: Vec<File> = Vec::new();
    while let Some(entry) = dir_iterator.next_entry().await? {
        let path = entry.path();
        content.push(File::from_path(&path)?);
    }

    Ok(Json(content))
}
