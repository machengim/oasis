use crate::entity::{state::State, token::Token, upload::BeforeUploadRequest};
use tide::{Request, Response, Result, StatusCode};

// Post "/api/file/before_upload"
pub async fn post_before_upload(mut req: Request<State>) -> Result {
    let token = Token::from_ext(&req)?;
    if token.permission <= 0 {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    let upload_req: BeforeUploadRequest = req.body_json().await?;
    let uuid = BeforeUploadRequest::prepare_tmp_dir(&req).await?;
    let task = upload_req.create_task(uuid, token.uid);
    req.state().add_task(task)?;

    Ok(Response::new(200))
}
