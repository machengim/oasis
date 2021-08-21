use crate::{
    entity::file::File,
    request::upload::{
        FinishUploadRequest, PrepareUploadRequest, SliceUploadQuery, SliceUploadRequest,
    },
    service::{state::State, token::Token},
    util::{self, db},
};
use tide::{convert::json, Request, Response, Result, StatusCode};

// Post "/api/upload/prepare"
pub async fn post_prepare_upload(mut req: Request<State>) -> Result {
    let upload_req = PrepareUploadRequest::from(&mut req).await?;
    if !upload_req.validate().await? {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !upload_req.auth(&req).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let storage = req.state().get_storage()?;
    let upload_id = util::prepare_tmp_dir(&storage).await?;
    let token = Token::from_ext(&req)?;
    let task = upload_req.create_task(&upload_id, token.uid);
    req.state().add_task(task)?;

    Ok(upload_id.into())
}

// Post "/api/upload/progress/:upload_id?index=0&hash=ie01rlm"
// Use upload_id in url to avoid invalid upload requests.
pub async fn post_upload(mut req: Request<State>) -> Result {
    let slice_req = SliceUploadRequest::from(&mut req).await?;

    if !slice_req.validate(&req)? {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    if !slice_req.auth(&req)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let storage = req.state().get_storage()?;
    slice_req.write_tmp_file(&storage).await?;
    req.state().update_task_index(&slice_req.upload_id)?;

    Ok(Response::new(200))
}

// Post "/api/upload/finish"
pub async fn post_finish_upload(mut req: Request<State>) -> Result {
    let finish_req = FinishUploadRequest::from(&mut req).await?;
    if !finish_req.auth(&req)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let mut task = req.state().find_upload_task_id(&finish_req.upload_id)?;
    let storage = req.state().get_storage()?;
    task.combine_slices(&storage).await?;

    let insert_file_query = task.insert_file_query()?;
    let mut conn = req.state().get_pool_conn().await?;
    let id = db::insert_single(insert_file_query, &mut conn).await?;
    req.state().remove_task(task)?;

    let file = File::get_file_by_id(id, &mut conn).await?;
    Ok(json!(file).into())
}
