mod api;
mod entity;
mod route;
mod util;
use entity::app::AppState;
use entity::query::Query;
use entity::site::Site;
use entity::upload::UploadState;
use rocket::fs::FileServer;
use sqlx::{Pool, Sqlite};
use std::sync::Mutex;
use util::db;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    let app_state = init_app_state().await;
    let upload_state = UploadState::new();
    let react_dir = util::get_react_dir();

    if let Err(e) = rocket::build()
        .manage(app_state)
        .manage(upload_state)
        .mount("/", route::serve_static_route())
        .mount("/api", api::serve_api())
        .mount("/", FileServer::from(react_dir))
        .launch()
        .await
    {
        println!("Starting server error: {:?}", e);
    }
}

async fn init_app_state() -> AppState {
    let pool = db::get_db_conn().await;
    let site = read_site_info(&pool).await;
    let first_run = site.first_run == 1;
    let secret = site.secret;
    let storage = site.storage;

    AppState {
        first_run: Mutex::new(first_run),
        pool,
        storage: Mutex::new(storage),
        secret: Mutex::new(secret),
    }
}

async fn read_site_info(pool: &Pool<Sqlite>) -> Site {
    let query = Query::new("SELECT * FROM site", vec![]);

    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(_) => panic!("Cannot get db connection"),
    };

    match db::fetch_single::<Site>(query, &mut conn).await {
        Ok(site) => site,
        Err(e) => panic!("Cannot read site info from db: {}", e),
    }
}
