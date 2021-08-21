use crate::entity::user::User;
use crate::request::site::SetupRequest;
use crate::service::init;
use crate::service::state::State;
use crate::util::{self, db};
use tide::{Request, Response, Result, StatusCode};

// post "api/setup"
pub async fn post_setup(mut req: Request<State>) -> Result {
    let mut setup_req = SetupRequest::from(&mut req).await?;
    if !setup_req.validate()? {
        return Ok(Response::new(StatusCode::BadRequest));
    }

    let mut conn = req.state().get_pool_conn().await?;
    if let Some(_) = User::find_exist_username(&setup_req.username, &mut conn).await? {
        return Ok(Response::new(StatusCode::Conflict));
    }

    let storage_path = init::create_site_dirs(&setup_req.storage).await?;
    let secret = util::generate_secret_key();

    setup_req.storage = storage_path.to_string_lossy().to_string();
    let insert_user_sql = User::from_setup_req(&setup_req).insert_user_query()?;
    let prepare_root_sql = setup_req.prepare_root_in_db_query(setup_req.time.unwrap());
    let setup_site_sql = setup_req.init_site_query(&secret);

    db::tx_execute(
        vec![insert_user_sql, prepare_root_sql, setup_site_sql],
        &mut conn,
    )
    .await?;

    let mut site = req.state().get_site_value()?;
    site.first_run = 0;
    site.storage = setup_req.storage.clone();
    site.secret = secret;
    req.state().set_site(site)?;

    Ok(Response::new(StatusCode::Ok))
}
