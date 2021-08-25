use crate::request::file::GetDirRequest;
use crate::service::state::State;
use crate::util::file_system;
use tide::{convert::json, Request, Response, Result, StatusCode};

// Get "/api/dir/:dir_path"
pub async fn get_dir_contents(mut req: Request<State>) -> Result {
    let dir_req = GetDirRequest::from(&mut req).await?;
    if !dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !dir_req.auth(&req)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let files = file_system::get_dir_content(dir_req.path.into(), false).await?;

    Ok(json!(files).into())
}
