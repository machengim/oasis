#[macro_use]
extern crate rocket;
mod api;
mod entity;
mod service;
mod util;
use entity::site::Site;
use rocket::fs::FileServer;
use service::app_state::AppState;
use service::fairings::CacheFairing;
use util::init;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    if !init::check_db_file() {
        init::create_db().await?;
    }

    let pool = init::get_db_pool().await?;
    let mut conn = pool.acquire().await?;

    let state = match Site::read(&mut conn).await? {
        Some(site) => AppState::new_with_site(site, pool),
        None => AppState::new_without_site(pool),
    };

    let server = rocket::build()
        .manage(state)
        .attach(CacheFairing)
        .mount("/api", api::serve())
        .mount("/", service::static_route::serve())
        .mount("/", FileServer::from(util::get_frontend_path()))
        .launch()
        .await;

    server.expect("server failed unexpectedly");

    Ok(())
}
