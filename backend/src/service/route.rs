use crate::service::{state::State, token::Token};
use crate::util::env;
use tide::{Body, Redirect, Request, Response, Result, StatusCode};

pub async fn get_index(req: Request<State>) -> Result {
    let state = req.state();
    let first_run = state.get_first_run()?;

    if first_run {
        Ok(Redirect::temporary("/setup").into())
    } else if Token::auth_user_permission(&req) <= 0 {
        Ok(Redirect::temporary("/login").into())
    } else {
        let mut res = Response::new(200);
        res.set_body(Body::from_file(env::get_front_index()?).await?);

        Ok(res)
    }
}

pub async fn get_login(_: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_file(env::get_front_index()?).await?);

    Ok(res)
}

pub async fn get_setup(req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;
    let mut res = Response::new(200);

    if first_run {
        res.set_body(Body::from_file(env::get_front_index()?).await?);
    } else {
        res.set_status(StatusCode::BadRequest)
    }

    Ok(res)
}
