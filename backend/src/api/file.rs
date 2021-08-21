use crate::entity::file::File;
use crate::request::file::DirListRequest;
use crate::service::state::State;
use tide::{convert::json, Request, Response, Result, StatusCode};

// Get "/api/dir/:dir_id".
pub async fn get_dir_list(req: Request<State>) -> Result {
    let dir_req = DirListRequest::from(&req)?;
    let mut conn = req.state().get_pool_conn().await?;

    if !dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !dir_req.auth(&mut conn).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let files = File::get_files_in_dir(dir_req.dir_id, dir_req.user_id, &mut conn).await?;

    Ok(json!(files).into())
}
