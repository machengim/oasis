mod entity;
mod util;
use entity::state::State;
use tide::{Body, Redirect, Request, Response, Result};

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

    let front_dir = util::get_front_dir()?;
    app.at("/").get(get_index);
    app.at("/login").get(get_login);
    // TODO: handler for get /setup
    app.at("/").serve_dir(front_dir)?;

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
