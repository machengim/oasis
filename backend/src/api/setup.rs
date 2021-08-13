use crate::api::setup;
use crate::entity::state::State;
use crate::entity::user::User;
use crate::util::db;
use crate::{entity::site::SetupRequest, util};
use tide::{Request, Response, Result, StatusCode};

// post "api/setup"
pub async fn post_setup(mut req: Request<State>) -> Result {
    let mut setup_req: SetupRequest = req.body_json().await?;
    let mut conn = req.state().get_pool_conn().await?;
    if let Some(_) = User::find_exist_username(&setup_req.username, &mut conn).await? {
        return Ok(Response::new(StatusCode::Conflict));
    }

    let storage_path = util::create_site_dirs(&setup_req.storage).await?;
    setup_req.storage = storage_path.to_string_lossy().to_string();
    let insert_user_sql = setup_req.init_admin_query()?;
    let setup_site_sql = setup_req.update_site_query();
    db::tx_execute(vec![insert_user_sql, setup_site_sql], &mut conn).await?;

    Ok(Response::new(StatusCode::Ok))
}
