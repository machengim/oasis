#[macro_use]
extern crate rocket;
mod api;
mod entity;
mod service;
mod util;
use entity::site::Site;
use rocket::fs::FileServer;
use service::app_state::AppState;
use service::fairings::StaticFileCache;
use util::{init, rocket_env::RocketEnv};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (ip, port) = match util::get_listen_ip_and_port() {
        Ok(Some(ret)) => ret,
        Ok(None) => print_help(),
        Err(e) => return Err(e),
    };

    init::init_app().await?;
    let pool = init::get_db_pool().await?;
    let mut conn = pool.acquire().await?;
    init::check_update(&mut conn).await?;

    let site_op = Site::read(&mut conn).await?;
    let state = AppState::new(site_op, pool);
    RocketEnv::new().setup();

    let figment = rocket::Config::figment()
        .merge(("address", &ip))
        .merge(("port", port));

    match rocket::custom(figment)
        .manage(state)
        .attach(StaticFileCache)
        .mount("/api", api::serve())
        .mount("/", service::static_route::serve())
        .mount("/", FileServer::from(util::get_frontend_path()))
        .ignite()
        .await
    {
        Ok(rocket) => {
            println!("Server running on {}:{}", ip, port);
            rocket.launch().await?;
        }
        Err(e) => match e.kind() {
            rocket::error::ErrorKind::Config(rocket::figment::error::Error {
                kind: rocket::figment::error::Kind::Message(mes),
                ..
            }) => {
                eprintln!("{}", mes);
                std::process::exit(1);
            }
            _ => {
                return Err(anyhow::anyhow!(e));
            }
        },
    };

    Ok(())
}

fn print_help() -> ! {
    println!("Usage: ./oasis [ip] [port]");
    std::process::exit(0);
}
