#[macro_use]
extern crate rocket;
mod api;
mod entity;
mod service;
mod util;
use entity::site::Site;
use local_ip_address;
use rocket::fs::FileServer;
use service::app_state::AppState;
use service::fairings::StaticFileCache;
use util::{init, local_ip, rocket_env::RocketEnv};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if !init::check_db_file() {
        init::create_db().await?;
    }

    let pool = init::get_db_pool().await?;
    let mut conn = pool.acquire().await?;
    let site_op = Site::read(&mut conn).await?;
    let state = AppState::new(site_op, pool);
    RocketEnv::new().setup();

    let rocket = rocket::build()
        .manage(state)
        .attach(StaticFileCache)
        .mount("/api", api::serve())
        .mount("/", service::static_route::serve())
        .mount("/", FileServer::from(util::get_frontend_path()))
        .ignite()
        .await?;

    let ip = local_ip::get()?;
    println!("Server running on {}:{}", ip, rocket.config().port);

    rocket.launch().await?;

    Ok(())
}
