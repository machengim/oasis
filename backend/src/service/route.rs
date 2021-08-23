use crate::entity::file::File;
use crate::service::{state::State, token::Token};
use crate::util::env;
use tide::{Body, Redirect, Request, Response, Result, Server, StatusCode};

pub fn mount_static(mut app: Server<State>) -> Server<State> {
    app.at("/").get(get_index);
    app.at("/index.html").get(get_index);
    app.at("/login").get(get_login);
    app.at("/setup").get(get_setup);
    app.at("/files").get(get_root_files);
    app.at("/files/:file_id").get(get_specific_file);
    // app.at("/detail/:file_id").get(get_specific_file);

    app
}

async fn get_index(req: Request<State>) -> Result {
    let state = req.state();
    let first_run = state.get_first_run()?;

    if first_run {
        Ok(Redirect::temporary("/setup").into())
    } else if Token::auth_user_permission(&req) <= 0 {
        Ok(Redirect::temporary("/login").into())
    } else {
        Ok(Redirect::temporary("/files").into())
    }
}

async fn get_login(_: Request<State>) -> Result {
    let mut res = Response::new(200);
    res.set_body(Body::from_file(env::get_front_index()?).await?);

    Ok(res)
}

async fn get_setup(req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;
    let mut res = Response::new(200);

    if first_run {
        res.set_body(Body::from_file(env::get_front_index()?).await?);
    } else {
        res.set_status(StatusCode::BadRequest)
    }

    Ok(res)
}

async fn get_root_files(req: Request<State>) -> Result {
    let token = Token::from_ext(&req)?;
    if token.uid <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let mut res = Response::new(200);
    res.set_body(Body::from_file(env::get_front_index()?).await?);

    Ok(res)
}

async fn get_specific_file(req: Request<State>) -> Result {
    let file_id: i64 = req.param("file_id")?.parse()?;
    let user_id = Token::from_ext(&req)?.uid;
    if user_id <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let mut conn = req.state().get_pool_conn().await?;
    let file_owner_id = File::find_file_owner(file_id, &mut conn).await?;

    if file_owner_id != user_id {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let mut res = Response::new(200);
    res.set_body(Body::from_file(env::get_front_index()?).await?);

    Ok(res)
}
