use crate::{
    entity::{
        file::File,
        state::State,
        token::Token,
        upload::{BeforeUploadRequest, FinishUploadRequest, SliceUploadQuery},
    },
    util::db,
};
use tide::{convert::json, Request, Response, Result, StatusCode};

// Post "/api/file/before-upload"
pub async fn post_before_upload(mut req: Request<State>) -> Result {
    let token = Token::from_ext(&req)?;
    if token.permission <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let upload_req: BeforeUploadRequest = req.body_json().await?;
    let storage = req.state().get_storage()?;
    let upload_id = BeforeUploadRequest::prepare_tmp_dir(&storage).await?;
    let task = upload_req.create_task(&upload_id, token.uid);
    req.state().add_task(task)?;

    Ok(upload_id.into())
}

// Post "/api/file/upload/:upload_id?index=0&hash=ie01rlm"
// Use upload_id in url to avoid invalid upload requests.
pub async fn post_upload(mut req: Request<State>) -> Result {
    let upload_id = req.param("upload_id")?.to_string();

    if !validate_upload(&req, &upload_id)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    // let slice_req: SliceUploadRequest = req.body_json().await?;
    let slice_query: SliceUploadQuery = req.query()?;
    let data = req.body_bytes().await?;
    if !slice_query.validate_hash(&data) {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let task = req.state().find_upload_task_id(&upload_id)?;
    if slice_query.index != task.current_index {
        return Ok(Response::new(StatusCode::Conflict));
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
    if !validate_upload(&req, &finish_req.upload_id)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let mut task = req.state().find_upload_task_id(&finish_req.upload_id)?;
    let storage = req.state().get_storage()?;
    task.combine_slices(&storage).await?;

    let insert_file_query = task.insert_file_query(0)?;
    let mut conn = req.state().get_pool_conn().await?;
    // db::execute(insert_file_query, &mut conn).await?;
    let id = db::insert_single(insert_file_query, &mut conn).await?;
    req.state().remove_task(task)?;

    Ok(id.to_string().into())
}

// Get "/api/file/dir/:dir_id".
pub async fn get_file_list(req: Request<State>) -> Result {
    let token = Token::from_ext(&req)?;
    if token.permission <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let dir_id: i64 = req.param("dir_id")?.parse()?;
    let mut conn = req.state().get_pool_conn().await?;
    let files = File::get_files_in_dir(dir_id, token.uid, &mut conn).await?;

    Ok(json!(files).into())
}

fn validate_upload(req: &Request<State>, upload_id: &str) -> anyhow::Result<bool> {
    let token = Token::from_ext(req)?;
    if token.permission <= 0 {
        return Ok(false);
    }

    let task = req.state().find_upload_task_id(upload_id)?;
    if task.owner_id != token.uid {
        return Ok(false);
    }

    Ok(true)
}
