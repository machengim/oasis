mod db;
mod entity;
mod utils;
use entity::Site;
use rocket::fs::FileServer;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use sqlx::{Pool, Sqlite};
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

// Pages need redirect: index, setup and dashboard. maybe file list pages as well.
// Some redirects could be handled in React.
#[get("/")]
fn index(dest: Destination) -> Redirect {
    match dest.option {
        1 => Redirect::to(uri!("/setup")),
        _ => Redirect::to(uri!("/login")),
    }
}

struct AppState {
    pub first_run: Mutex<bool>,
    pub pool: Mutex<Pool<Sqlite>>,
}

struct Destination {
    pub option: i8,
}

#[derive(Debug)]
enum CustomError {
    BadRequest,
    InternalError,
}

// The URL redirect relies on three parts: first_run, request path, and the user role.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Destination {
    type Error = CustomError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        let first_run = state.first_run.lock().unwrap();

        match *first_run {
            false => Outcome::Forward(()),
            true => Outcome::Success(Destination { option: 1 }),
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
        .mount("/", routes![index])
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
