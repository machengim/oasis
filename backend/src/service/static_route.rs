use super::error::Error;
use super::token::Token;
use crate::service::app_state::AppState;
use crate::util;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::{Either, Route, Shutdown, State};
use std::path::PathBuf;

pub fn serve() -> Vec<Route> {
    routes![index, index_html, shutdown, login, setup, files, files_all]
}

#[get("/")]
fn index(state: &State<AppState>) -> Redirect {
    handle_index(state.get_first_run())
}

#[get("/index.html")]
fn index_html(state: &State<AppState>) -> Redirect {
    handle_index(state.get_first_run())
}

fn handle_index(first_run: bool) -> Redirect {
    if first_run {
        Redirect::temporary(uri!("/setup"))
    } else {
        Redirect::temporary(uri!("/files"))
    }
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    open_index_page().await
}

#[get("/setup")]
async fn setup(state: &State<AppState>) -> Result<Option<NamedFile>, Error> {
    match state.get_first_run() {
        true => Ok(open_index_page().await),
        false => Err(Error::Forbidden),
    }
}

#[get("/files")]
async fn files(token: Token) -> Either<Option<NamedFile>, Redirect> {
    handle_files(token).await
}

#[get("/files/<_dirs..>")]
async fn files_all(token: Token, _dirs: PathBuf) -> Either<Option<NamedFile>, Redirect> {
    handle_files(token).await
}

async fn handle_files(token: Token) -> Either<Option<NamedFile>, Redirect> {
    if token.uid > 0 && token.permission > 0 {
        Either::Left(open_index_page().await)
    } else {
        Either::Right(Redirect::temporary(uri!("/login")))
    }
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown, token: Token) -> Result<&'static str, Error> {
    if token.uid <= 0 || token.permission < 9 {
        return Err(Error::Forbidden);
    }

    shutdown.notify();
    Ok("The server is shutting down...")
}

async fn open_index_page() -> Option<NamedFile> {
    let index = util::get_frontend_path().join("index.html");
    NamedFile::open(index).await.ok()
}
