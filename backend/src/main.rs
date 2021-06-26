mod db;
mod entity;
mod server;
mod utils;
use warp::{http::Uri, Filter};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();

    let dir = utils::get_config_dir();
    debug!("Using directory: {:?}", &dir);
    let pool = db::get_db_conn(&dir).await;

    server::run(pool).await;
}
