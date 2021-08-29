mod api;
mod entity;
mod request;
mod service;
mod util;
use service::{middleware, route, state::State};
use util::{env, init};

use tide::Result;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();
    dotenv::dotenv().ok();

    if !init::check_app_installed() {
        if let Err(e) = init::init_app_db().await {
            eprintln!("Cannot create database: {}", e);
            std::process::exit(1);
        }
    }

    let pool = match init::get_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Cannot get connection pool: {}", e);
            std::process::exit(1);
        }
    };

    let state = State::init(pool).await;
    let mut app = tide::with_state(state);
    app.with(middleware::load_token);

    app = api::mount_api(app);
    app = route::mount_static(app);

    // Mount static folder
    let front_dir = env::get_front_dir()?;
    app.at("/").serve_dir(front_dir)?;

    let address = init::get_listen_address();
    app.listen(address).await?;

    Ok(())
}
