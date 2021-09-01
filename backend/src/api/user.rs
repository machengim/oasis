use crate::entity::user::User;
use crate::service::app_state::AppState;
use crate::service::error::Error;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize};
use rocket::{Route, State};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn route() -> Vec<Route> {
    routes![login]
}

#[post("/login", data = "<req_body>")]
async fn login(
    state: &State<AppState>,
    req_body: Json<LoginRequest>,
    jar: &CookieJar<'_>,
) -> Result<(), Error> {
    if req_body.username.len() < 2 || req_body.password.len() < 6 {
        return Err(Error::BadRequest);
    }

    let mut conn = state.get_pool_conn().await?;
    let user = User::login(&req_body, &mut conn).await?;
    let secret = state.get_secret()?;
    let token_str = user.generate_token().encode(&secret)?;

    let cookie = Cookie::build("token", token_str)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(7))
        .finish();

    jar.add(cookie);

    Ok(())
}
