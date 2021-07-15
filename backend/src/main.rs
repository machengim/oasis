mod api;
pub mod auth;
mod db;
mod entity;
mod filesystem;
mod route;
mod util;
use entity::Site;
use rocket::fs::FileServer;
use sqlx::{Pool, Sqlite};
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    let state = init_app_state().await;
    let react_dir = util::get_react_dir();

    if let Err(e) = rocket::build()
        .manage(state)
        .mount("/", route::serve_route())
        .mount("/api", api::serve_api())
        .mount("/", FileServer::from(react_dir))
        .launch()
        .await
    {
        println!("Starting server error: {:?}", e);
    }
}

async fn init_app_state() -> entity::AppState {
    let pool = db::get_db_conn().await;
    let site = read_site(&pool).await;
    let first_run = site.first_run == 1;

    entity::AppState {
        first_run: Mutex::new(first_run),
        pool,
        storage: Mutex::new(String::new()),
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
