use crate::util::file_system;
use crate::{request::sys::SysDirsRequest, service::state::State};
use tide::{convert::json, Request, Response, Result, StatusCode};

// get "/api/sys/volumes"
pub async fn get_system_volumes(req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;

    if !first_run {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let volumes = file_system::get_system_volumes()?;
    let res = Response::builder(200).body(json!(volumes)).build();

    Ok(res)
}

// get "/api/sys/dirs/:dir"
pub async fn get_system_dirs(req: Request<State>) -> Result {
    let dir_req = SysDirsRequest::from(&req)?;

    if !dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !dir_req.auth(&req)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let sub_dirs = file_system::get_system_dirs(&dir_req.path).await?;

    Ok(json!(sub_dirs).into())
}
