use crate::entity::user::User;
use crate::service::{db, state::State};
use crate::util::init;
use crate::{request::site::SiteSetupRequest, util};
use tide::{Request, Response, Result, StatusCode};

// Post "/api/site"
pub async fn post_setup(mut req: Request<State>) -> Result {
    let first_run = req.state().get_first_run()?;
    if !first_run {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let mut setup_req = SiteSetupRequest::from(&mut req).await?;
    if !setup_req.validate()? {
        return Ok(Response::new(StatusCode::BadRequest));
    }
    let mut conn = req.state().get_pool_conn().await?;
    if User::find_exist_username(&setup_req.username, &mut conn)
        .await?
        .is_some()
    {
        return Ok(Response::new(StatusCode::Conflict));
    }

    let root_path = init::create_site_dir(&setup_req.storage).await?;
    setup_req.storage = root_path.to_string_lossy().to_string();
    let secret = util::generate_secret_key();
    let site = setup_req.to_site(&secret);
    let new_site_query = site.create_query();
    let new_user_query = setup_req.to_admin().insert_query()?;

    db::tx_execute(vec![new_user_query, new_site_query], &mut conn).await?;
    req.state().set_site(site)?;

    Ok("".into())
}
