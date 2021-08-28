use crate::entity::file::File;
use crate::request::file::CreateDirRequest;
use crate::service::state::State;
use crate::{request::file::GetDirRequest, service::token::Token};
use sqlx::Acquire;
use tide::{convert::json, Request, Response, Result, StatusCode};

#[derive(serde::Serialize)]
struct DirContentResponse {
    dir: File,
    contents: Vec<File>,
}

// Get "/api/dir/:dir_path"
pub async fn get_dir_contents(mut req: Request<State>) -> Result {
    if Token::auth_user_permission(&req) <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let dir_req = GetDirRequest::from_req(&mut req).await?;
    let user_id = Token::from_ext(&req)?.uid;
    let mut conn = req.state().get_pool_conn().await?;
    let dir = match File::get_dir_by_path(user_id, &dir_req.path, &mut conn).await? {
        Some(v) => v,
        None => return Ok(Response::new(StatusCode::NotFound)),
    };
    let contents = dir.read_dir_contents(user_id, &mut conn).await?;

    let res = DirContentResponse { dir, contents };

    Ok(json!(res).into())
}

// Post "/api/dir"
pub async fn post_dir(mut req: Request<State>) -> Result {
    if Token::auth_user_permission(&req) <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let create_dir_req = CreateDirRequest::from_req(&mut req).await?;
    if !create_dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let user_id = Token::from_ext(&req)?.uid;
    let mut conn = req.state().get_pool_conn().await?;
    let dir = match File::get_dir_by_path(user_id, &create_dir_req.paths, &mut conn).await? {
        Some(v) => v,
        None => return Ok(Response::new(StatusCode::NotFound)),
    };

    if dir.owner_id != user_id {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    create_dir_req.create_dir(&mut req).await?;
    let mut tx = conn.begin().await?;
    let dir_id = dir
        .create_sub_dir(&create_dir_req.dir_name)
        .create_query(&mut tx)
        .await?;
    tx.commit().await?;

    let file = File::read_file_by_id(dir_id, &mut conn).await?;

    Ok(json!(file).into())
}
