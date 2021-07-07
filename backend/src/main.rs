mod db;
mod entity;
mod filesystem;
mod utils;
use entity::Site;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

// Pages need redirect: index, setup and dashboard. maybe file list pages as well.
// Some redirects could be handled in React.
#[get("/")]
fn index(dest: Auth) -> Redirect {
    redirect_index(dest)
}

#[get("/index.html")]
fn index_html(auth: Auth) -> Redirect {
    redirect_index(auth)
}

fn redirect_index(auth: Auth) -> Redirect {
    match auth.option {
        1 => Redirect::to(uri!("/setup")),
        _ => Redirect::to(uri!("/login")),
    }
}

#[get("/setup")]
fn setup(_auth: Auth) {
    // This handler has only forward and forbidden, no real handling process needed.
}

#[get("/setup/index.html")]
fn setup_index(_auth: Auth) {}

#[get("/fs/volumes")]
fn system_volumes(_auth: Auth) -> Result<Json<Vec<String>>, Status> {
    match filesystem::get_system_volumes() {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(Status::NotImplemented),
    }
}

// TODO: Should prevent reading dirs outside of the mountpoints.
#[get("/fs/dirs/<dir..>")]
async fn system_sub_dirs(dir: PathBuf) -> Result<Json<Vec<String>>, Status> {
    println!(" ============ Got dir {:?} ===============", &dir);
    match filesystem::get_system_dirs(dir).await {
        Ok(v) => Ok(Json(v)),
        Err(e) => {
            eprintln!("Got error when retrieving sub dirs: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

pub struct AppState {
    pub first_run: Mutex<bool>,
    pub pool: Mutex<Pool<Sqlite>>,
}

pub struct Auth {
    pub option: i8,
}

// The URL redirect relies on three parts: first_run, request path, and the user role.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        let first_run = state.first_run.lock().unwrap();
        let url = req.uri().path().to_string();

        // Could combine all failure arms to a single one.
        match (&url[..], *first_run) {
            ("/", false) | ("/index.html", false) => Outcome::Forward(()),
            ("/", true) | ("/index.html", true) => Outcome::Success(Auth { option: 1 }),
            ("/setup", true) | ("/setup/index.html", true) => Outcome::Forward(()),
            ("/setup", false) | ("/setup/index.html", false) => {
                Outcome::Failure((Status::Forbidden, Status::Forbidden))
            }
            ("/api/fs/volumes", true) => Outcome::Success(Auth { option: 1 }),
            ("/api/fs/volumes", false) => Outcome::Failure((Status::Forbidden, Status::Forbidden)),
            _ => Outcome::Failure((Status::Forbidden, Status::Forbidden)),
        }
    }
}

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    let state = init_app_state().await;
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");

    if let Err(e) = rocket::build()
        .manage(state)
        .mount("/", routes![index, index_html, setup, setup_index])
        .mount("/api", routes![system_volumes, system_sub_dirs])
        .mount("/", FileServer::from(react_dir))
        .launch()
        .await
    {
        println!("Starting server error: {:?}", e);
    }
}

async fn init_app_state() -> AppState {
    let pool = db::get_db_conn().await;
    let site = read_site(&pool).await;
    let first_run = site.first_run == 1;

    AppState {
        first_run: Mutex::new(first_run),
        pool: Mutex::new(pool),
    }
}

async fn read_site(pool: &Pool<Sqlite>) -> Site {
    let sql = "SELECT * FROM site";
    let args = vec![];

    match db::fetch_single::<Site>(sql, args, pool).await {
        Ok(site) => site,
        Err(e) => panic!("Cannot read site info from db: {}", e),
    }
}
