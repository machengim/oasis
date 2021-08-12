use crate::entity::site::SetupRequest;
use crate::entity::state::State;
use crate::entity::user::User;
use crate::util::custom_error::CustomError;
use crate::util::db;
use tide::{Request, Response, Result, StatusCode};

pub async fn post_setup(mut req: Request<State>) -> Result {
    let setup_req: SetupRequest = req.body_json().await?;
    let mut conn = req.state().get_pool_conn().await?;
    if let Some(_) = User::find_exist_username(&setup_req.username, &mut conn).await? {
        return Err(CustomError::from(StatusCode::Conflict, "User existed"));
    }

    let insert_user_sql = setup_req.init_admin_query()?;
    let setup_site_sql = setup_req.update_site_query();
    db::tx_execute(vec![insert_user_sql, setup_site_sql], &mut conn).await?;

    Ok(Response::new(StatusCode::Accepted))
}
