mod api;
mod db;
mod entity;
mod filesystem;
mod server;
mod utils;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();
    let pool = db::get_db_conn().await;
    server::run(pool).await;
}
