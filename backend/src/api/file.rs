use crate::request::file::{DeleteFileRequest, DirListRequest, RenameFileRequest};
use crate::service::state::State;
use crate::util::db;
use crate::{entity::file::File, request::file::CreateDirRequest};
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

// Post "/api/dir"
pub async fn post_create_dir(mut req: Request<State>) -> Result {
    let new_dir_req = CreateDirRequest::from(&mut req).await?;
    let mut conn = req.state().get_pool_conn().await?;
    if !new_dir_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !new_dir_req.auth(&mut conn).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let new_dir = File::from_create_dir_req(&new_dir_req)
        .insert_to_db(&mut conn)
        .await?;

    Ok(json!(new_dir).into())
}

// Put "/api/file/:file_id"
pub async fn put_rename_file(mut req: Request<State>) -> Result {
    let rename_req = RenameFileRequest::from(&mut req).await?;
    let mut conn = req.state().get_pool_conn().await?;
    if !rename_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !rename_req.auth(&mut conn).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    db::execute(rename_req.to_query(), &mut conn).await?;

    Ok(Response::new(200))
}

// Delete "/api/file/:file_id"
pub async fn delete_file(req: Request<State>) -> Result {
    let delete_req = DeleteFileRequest::from(&req)?;
    let mut conn = req.state().get_pool_conn().await?;

    if !delete_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !delete_req.auth(&mut conn).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let storage = req.state().get_storage()?;
    let file = File::get_file_by_id(delete_req.file_id, &mut conn).await?;
    file.delete(&storage, &mut conn).await?;

    Ok(Response::new(200))
}
