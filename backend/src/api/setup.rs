use crate::entity::state::State;
use crate::entity::user::{LoginRequest, User};
use crate::util::db;
use crate::{entity::site::SetupRequest, util};
use tide::http::Cookie;
use tide::{Request, Response, Result, StatusCode};

// post "api/setup"
pub async fn post_setup(mut req: Request<State>) -> Result {
    let mut setup_req: SetupRequest = req.body_json().await?;
    let mut conn = req.state().get_pool_conn().await?;
    if let Some(_) = User::find_exist_username(&setup_req.username, &mut conn).await? {
        return Ok(Response::new(StatusCode::Conflict));
    }

    let storage_path = util::create_site_dirs(&setup_req.storage).await?;
    let secret = util::generate_secret_key();

    setup_req.storage = storage_path.to_string_lossy().to_string();
    let insert_user_sql = setup_req.init_admin_query()?;
    let setup_site_sql = setup_req.update_site_query(&secret);
    db::tx_execute(vec![insert_user_sql, setup_site_sql], &mut conn).await?;

    let mut site = req.state().get_site_value()?;
    site.first_run = 0;
    site.storage = setup_req.storage.clone();
    site.secret = secret;
    req.state().set_site(site)?;

    Ok(Response::new(StatusCode::Ok))
}

pub async fn login(mut req: Request<State>) -> Result {
    let login_req: LoginRequest = req.body_json().await?;
    let mut conn = req.state().get_pool_conn().await?;
    let secret = req.state().get_secret()?;

    let user = login_req.login(&mut conn).await?;
    let token_str = user.generate_token().encode(&secret)?;
    let token_expire_days = util::must_get_env_value("TOKEN_EXPIRE_DAYS", 7);
    let cookie = Cookie::build("token", token_str)
        .http_only(true)
        .path("/")
        .max_age(time::Duration::days(token_expire_days))
        .finish();

    let mut res = Response::new(200);
    res.insert_cookie(cookie);

    Ok(res)
}
