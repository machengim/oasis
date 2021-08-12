use tide::{convert::json, Request, Response, Result, StatusCode};

use crate::{
    entity::state::State,
    util::{custom_error::CustomError, file_system},
};

pub async fn get_system_volumes(req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;

    if !first_run {
        return Err(CustomError::from(
            StatusCode::BadRequest,
            "No permission to read volumes",
        ));
    }

    let volumes = file_system::get_system_volumes()?;
    let res = Response::builder(200).body(json!(volumes)).build();

    Ok(res)
}

// mount at "/api/ays/dirs/:dir*"
pub async fn get_system_dirs(req: Request<State>) -> Result {
    let dir = req.param("dir")?;
    println!("Get dir : {}", dir);

    Ok(Response::new(StatusCode::Accepted))
}
