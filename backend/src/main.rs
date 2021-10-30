#[macro_use]
extern crate rocket;
mod api;
mod entity;
mod service;
mod util;
use crate::util::local_ip::ServerConfig;
use entity::site::Site;
use rocket::fs::FileServer;
use service::app_state::AppState;
use service::fairings::StaticFileCache;
use std::{thread, time};
use util::{init, local_ip, rocket_env::RocketEnv};

#[tokio::main]
async fn main() {
    if let Err(e) = launch().await {
        eprintln!("{}", e);

        // For windows users, persist the window for 5 seconds to read the error message.
        if std::env::consts::OS.to_lowercase() == "windows" {
            let five_seconds = time::Duration::from_secs(5);
            thread::sleep(five_seconds);
        }
    }

    println!("Server shutdown");
}

async fn launch() -> Result<(), anyhow::Error> {
    init::init_app().await?;
    let pool = init::get_db_pool().await?;
    let mut conn = pool.acquire().await?;
    init::check_update(&mut conn).await?;

    let site_op = Site::read(&mut conn).await?;
    let state = AppState::new(site_op, pool);
    let config = ServerConfig::new()?;
    RocketEnv::setup(&config);

    let rocket = rocket::build()
        .manage(state)
        .attach(StaticFileCache)
        .mount("/api", api::serve())
        .mount("/", service::static_route::serve())
        .mount("/", FileServer::from(util::get_frontend_path()))
        .ignite()
        .await?;

    local_ip::show(&config)?;
    rocket.launch().await?;

    Ok(())
}
