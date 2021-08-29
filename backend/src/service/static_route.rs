use crate::service::app_state::AppState;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::{Route, Shutdown, State};

pub fn serve() -> Vec<Route> {
    routes![index, index_html, shutdown, login, setup]
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
        Redirect::temporary(uri!("/login"))
    }
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    open_index_page().await
}

#[get("/setup")]
async fn setup() -> Option<NamedFile> {
    open_index_page().await
}

async fn open_index_page() -> Option<NamedFile> {
    let index = "../frontend/public/index.html";
    NamedFile::open(index).await.ok()
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}
