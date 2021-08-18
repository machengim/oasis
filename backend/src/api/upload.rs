use crate::{
    entity::{
        file::File,
        upload::{BeforeUploadRequest, FinishUploadRequest, SliceUploadQuery},
    },
    service::{state::State, token::Token},
    util::{self, db},
};
use tide::{convert::json, Request, Response, Result, StatusCode};

// Post "/api/file/before-upload"
pub async fn post_before_upload(mut req: Request<State>) -> Result {
    let upload_req: BeforeUploadRequest = req.body_json().await?;
    if !upload_req.validate(&req).await? {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let storage = req.state().get_storage()?;
    let upload_id = util::prepare_tmp_dir(&storage).await?;

    let token = Token::from_ext(&req)?;
    let task = upload_req.create_task(&upload_id, token.uid);
    req.state().add_task(task)?;

    Ok(upload_id.into())
}

// Post "/api/file/upload/:upload_id?index=0&hash=ie01rlm"
// Use upload_id in url to avoid invalid upload requests.
pub async fn post_upload(mut req: Request<State>) -> Result {
    let upload_id = req.param("upload_id")?.to_string();
    let slice_query: SliceUploadQuery = req.query()?;
    let data = req.body_bytes().await?;

    if !slice_query.validate(&data, &req, &upload_id)? {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let storage = req.state().get_storage()?;
    slice_query
        .write_tmp_file(&data, &storage, &upload_id)
        .await?;
    req.state().update_task_index(&upload_id)?;

    Ok(Response::new(200))
}

// Post "/api/file/finish-upload"
pub async fn post_finish_upload(mut req: Request<State>) -> Result {
    let finish_req: FinishUploadRequest = req.body_json().await?;
    if !finish_req.validate(&req)? {
        return Ok(Response::new(StatusCode::BadRequest));
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
