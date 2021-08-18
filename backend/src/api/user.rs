use crate::entity::file::File;
use crate::entity::user::LoginRequest;
use crate::service::state::State;
use crate::util;
use tide::http::Cookie;
use tide::{Request, Response, Result};
use util::env;

// Post "/api/login".
pub async fn login(mut req: Request<State>) -> Result {
    let login_req: LoginRequest = req.body_json().await?;
    let mut conn = req.state().get_pool_conn().await?;
    let secret = req.state().get_secret()?;

    let user = login_req.login(&mut conn).await?;
    let token_str = user.generate_token().encode(&secret)?;
    let token_expire_days = env::must_get_env_value("TOKEN_EXPIRE_DAYS", 7);
    let cookie = Cookie::build("token", token_str)
        .http_only(true)
        .path("/")
        .max_age(time::Duration::days(token_expire_days))
        .finish();

    let root_id = File::find_root_dir(user.user_id, &mut conn).await?;
    let mut res = Response::new(200);
    res.insert_cookie(cookie);
    res.set_body(root_id.to_string());

    Ok(res)
}
