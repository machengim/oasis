use crate::entity::file::File;
use crate::service::{state::State, token::Token};
use tide::{convert::json, Request, Response, Result, StatusCode};

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
