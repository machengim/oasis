mod db;
mod entity;
mod filesystem;
mod utils;
use entity::Site;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{FileServer, NamedFile};
use rocket::http::uri::Origin;
use rocket::http::{Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{Data, Response};

use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(_auth: Auth) -> Redirect {
    Redirect::to(uri!("/setup"))
}

#[get("/setup")]
async fn setup(_auth: Auth) -> Option<NamedFile> {
    get_react_index().await
}

async fn get_react_index() -> Option<NamedFile> {
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");
    let mut path = PathBuf::from(react_dir);
    path.push("index.html");
    NamedFile::open(path).await.ok()
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    get_react_index().await
}

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
        let path = req.uri().path().to_string();

        match (&path[..], *first_run) {
            ("/", false) | ("/index.html", false) => Outcome::Forward(()),
            ("/", true) | ("/index.html", true) => Outcome::Success(Auth { option: 1 }),
            ("/setup", true) => Outcome::Success(Auth { option: 1 }),
            ("/setup", false) => Outcome::Failure((Status::Forbidden, Status::Forbidden)),
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
        //.attach(ServerRoute)
        //.attach(CacheContent)
        .mount("/", routes![index, login, setup])
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

/* Use in production mode.
struct CacheContent;

#[rocket::async_trait]
impl Fairing for CacheContent {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Get {
            response.set_raw_header("Cache-control", "private");
            response.set_raw_header("Cache-control", "max-age=3600");
            println!("------ Header set --------");
        }
    }
}
*/

struct ServerRoute;

#[rocket::async_trait]
impl Fairing for ServerRoute {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if request.method() == Method::Get {
            let request_url = request.uri().to_string();
            let target_uri = match &request_url[..] {
                "/" | "/index.html" => "/?page=home",
                "/setup" => "/?page=setup",
                "/login" => "/?page=login",
                _ => "",
            };

            if target_uri.len() > 0 {
                request.set_uri(Origin::parse(target_uri).unwrap());
                println!("New uri set: {}", request.uri());
            }
        }
    }
}
