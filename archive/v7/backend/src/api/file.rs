use crate::entity::file::FileListResponse;
use crate::request::file::{DeleteFileRequest, GetFileRequest, RenameFileRequest};
use crate::service::state::State;
use crate::util::{self, db};
use crate::{entity::file::File, request::file::CreateDirRequest};
use tide::{convert::json, Request, Response, Result, StatusCode};

// Get "/api/file/:file_id".
pub async fn get_file(req: Request<State>) -> Result {
    let get_file_req = GetFileRequest::from(&req)?;
    let mut conn = req.state().get_pool_conn().await?;

    if !get_file_req.validate() {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    if !get_file_req.auth(&mut conn).await? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let target_file = File::get_file_by_id(get_file_req.file_id, &mut conn).await?;

    match &target_file.file_type.to_lowercase()[..] {
        "root" | "dir" => {
            let files =
                File::get_files_in_dir(get_file_req.file_id, get_file_req.user_id, &mut conn)
                    .await?;
            let dirs = File::get_all_parents(get_file_req.file_id, &mut conn).await?;
            let res = FileListResponse { files, dirs };
            Ok(json!(res).into())
        }
        _ => Ok(get_range_file(&req, target_file).await?),
    }
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

async fn get_range_file(req: &Request<State>, file_in: File) -> Result {
    use crate::entity::range::{get_header_contents, get_range_length, read_file_meta};

    let range_header = req.header("Range");
    let storage = req.state().get_storage()?;
    let path = util::env::get_files_dir(&storage).join(&file_in.path);
    let (mut file, size) = match read_file_meta(&path) {
        Ok((file, size)) => (file, size),
        Err(_) => return Ok(Response::new(StatusCode::InternalServerError)),
    };
    let file_type = file_in.to_http_type();

    let (start, end) = get_range_length(size, range_header);
    let len = end - start + 1;

    let (contents, content_range) = match get_header_contents(start, end, size, &mut file) {
        Ok((v, l)) => (v, l),
        Err(_) => return Ok(Response::new(StatusCode::InternalServerError)),
    };

    println!("Content type: {:?}", &file_type);
    let mut res = Response::new(StatusCode::PartialContent);
    res.insert_header("Content-Type", file_type);
    res.insert_header("Content-Range", content_range);
    res.insert_header("Content-Length", len.to_string());
    res.set_body(contents);

    Ok(res)
}
