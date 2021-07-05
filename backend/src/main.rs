mod api;
mod db;
mod entity;
mod filesystem;
mod utils;
use actix_files::{Files, self};
use actix_web::{web, App, HttpServer};
use entity::{AppState, Site};
use std::sync::Mutex;
use sqlx::{Pool, Sqlite};

#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();

    let state = web::Data::new(init_app_state().await);
    debug!("app state: {:?}", &state);
    let react_dir = std::env::var("REACT_DIR").expect("Cannot get frontend dir from env");
    HttpServer::new(move || {
        App::new().app_data(state.clone())
            .service(api::index)
            .service(Files::new("/", react_dir.clone()).index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn init_app_state() -> AppState {
    let pool = db::get_db_conn().await;
    let site = read_site(&pool).await;
    let first_run = site.first_run == 1;

    AppState{first_run: Mutex::new(first_run), pool: Mutex::new(pool), storage: Mutex::new(site.storage)}
}

async fn read_site(pool: &Pool<Sqlite>) -> Site {
    let sql = "SELECT * FROM site";
    let args = vec![];

    match db::fetch_single::<Site>(sql, args, pool).await {
        Ok(site) => site,
        Err(e) => panic!("Cannot read configuration: {}", e),
    }
}