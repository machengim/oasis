use crate::service::state::State;
use crate::util::file_system;
use crate::{request::file::GetDirRequest, service::token::Token};
use tide::{convert::json, Request, Response, Result, StatusCode};

// Get "/api/dir/:dir_path"
pub async fn get_dir_contents(mut req: Request<State>) -> Result {
    if Token::auth_user_permission(&req) <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let dir_req = GetDirRequest::from(&mut req).await?;
    if !dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let files = file_system::get_dir_content(dir_req.path.into(), false).await?;

    Ok(json!(files).into())
}
