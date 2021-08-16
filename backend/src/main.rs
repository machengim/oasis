mod api;
mod entity;
mod util;
use entity::{state::State, token::Token};
use tide::{Body, Redirect, Request, Response, Result, StatusCode};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    dotenv::dotenv().ok();
    if !util::check_installed() {
        if let Err(e) = util::create_db_file().await {
            eprintln!("Cannot create database: {}", e);
            std::process::exit(1);
        }
    }

    let pool = match util::get_conn_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Cannot get connection pool: {}", e);
            std::process::exit(1);
        }
    };

    let state = State::init(pool).await;
    let mut app = tide::with_state(state);

    // Attach middlewares
    app.with(util::middleware::load_token);

    // Mount API route
    // Bug of Tide: nested route not working well with serve_dir().
    // app.at("/api").nest(api::api_route(state.clone()));
    app.at("/api/setup").post(api::setup::post_setup);
    app.at("/api/login").post(api::setup::login);
    app.at("/api/sys/volumes").get(api::sys::get_system_volumes);
    app.at("api/sys/dirs/:dir").get(api::sys::get_system_dirs);
    app.at("/api/file/before-upload")
        .post(api::file::post_before_upload);
    app.at("/api/file/upload/:upload_id")
        .post(api::file::post_upload);
    app.at("/api/file/finish-upload")
        .post(api::file::post_finish_upload);
    app.at("/api/file/dir/:dir_id")
        .get(api::file::get_file_list);

    // Mount static html page route
    app.at("/").get(get_index);
    app.at("/index.html").get(get_index);
    app.at("/login").get(get_login);
    app.at("/setup").get(get_setup);

    // Mount static folder
    let front_dir = util::get_front_dir()?;
    app.at("/").serve_dir(front_dir)?;

    // Launch app
    let address = util::get_listen_address();
    app.listen(address).await?;

    Ok(())
}

// TODO: auth user or redirect to /login.
async fn get_index(req: Request<State>) -> Result {
    let state = req.state();
    let first_run = state.get_first_run()?;

    if first_run {
        Ok(Redirect::temporary("/setup").into())
    } else if Token::auth_user_permission(&req) <= 0 {
        Ok(Redirect::temporary("/login").into())
    } else {
        let mut res = Response::new(200);
        res.set_body(Body::from_file(util::get_front_index()?).await?);

        Ok(res)
    }
}

async fn get_login(_: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_file(util::get_front_index()?).await?);

    Ok(res)
}

async fn get_setup(req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;
    let mut res = Response::new(200);

    if first_run {
        res.set_body(Body::from_file(util::get_front_index()?).await?);
    } else {
        res.set_status(StatusCode::BadRequest)
    }

    Ok(res)
}
