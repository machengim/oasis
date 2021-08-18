mod api;
mod entity;
mod service;
mod util;
use service::{init, route, state::State};
use util::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    dotenv::dotenv().ok();
    if !init::check_installed() {
        if let Err(e) = init::create_db_file().await {
            eprintln!("Cannot create database: {}", e);
            std::process::exit(1);
        }
    }

    let pool = match init::get_conn_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Cannot get connection pool: {}", e);
            std::process::exit(1);
        }
    };

    let state = State::init(pool).await;
    let mut app = tide::with_state(state);

    // Attach middlewares
    app.with(service::middleware::load_token);

    // Mount API route
    // Bug of Tide: nested route not working well with serve_dir().
    // app.at("/api").nest(api::api_route(state.clone()));
    app.at("/api/setup").post(api::site::post_setup);
    app.at("/api/login").post(api::user::login);
    app.at("/api/sys/volumes").get(api::sys::get_system_volumes);
    app.at("api/sys/dirs/:dir").get(api::sys::get_system_dirs);
    app.at("/api/file/before-upload")
        .post(api::upload::post_before_upload);
    app.at("/api/file/upload/:upload_id")
        .post(api::upload::post_upload);
    app.at("/api/file/finish-upload")
        .post(api::upload::post_finish_upload);
    app.at("/api/file/dir/:dir_id")
        .get(api::file::get_file_list);

    // Mount static html page route
    app.at("/").get(route::get_index);
    app.at("/index.html").get(route::get_index);
    app.at("/login").get(route::get_login);
    app.at("/setup").get(route::get_setup);

    // Mount static folder
    let front_dir = env::get_front_dir()?;
    app.at("/").serve_dir(front_dir)?;

    // Launch app
    let address = init::get_listen_address();
    app.listen(address).await?;

    Ok(())
}
