use crate::entity::user::User;
use crate::service::app_state::AppState;
use crate::service::auth::AuthUser;
use crate::service::error::Error;
use crate::service::token::Token;
use crate::util::constants::*;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Route, State};
use sqlx::Connection;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangePasswordRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub username: String,
    pub permission: i8,
}

pub fn route() -> Vec<Route> {
    routes![login, signout, change_password]
}

#[post("/login", data = "<req_body>")]
async fn login(
    state: &State<AppState>,
    req_body: Json<LoginRequest>,
    jar: &CookieJar<'_>,
) -> Result<Json<LoginResponse>, Error> {
    if req_body.username.len() < 2 || req_body.password.len() < 6 {
        return Err(Error::BadRequest);
    }

    let mut conn = state.get_pool_conn().await?;
    let user = User::login(&req_body.username, &req_body.password, &mut conn).await?;
    let secret = state.get_secret()?;
    let token_str = user.generate_token().encode(&secret)?;

    let cookie_token = Cookie::build(ACCESS_TOKEN, token_str)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(7))
        .finish();

    jar.add(cookie_token);
    let login = LoginResponse {
        username: user.username,
        permission: user.permission,
    };

    Ok(Json(login))
}

#[put("/user/password", data = "<req_body>")]
async fn change_password(
    state: &State<AppState>,
    _user: AuthUser,
    req_body: Json<ChangePasswordRequest>,
    jar: &CookieJar<'_>,
) -> Result<(), Error> {
    let mut conn = state.get_pool_conn().await?;
    let mut user = User::login(&req_body.username, &req_body.old_password, &mut conn).await?;
    user.password = req_body.new_password.clone();

    let mut tx = conn.begin().await?;
    user.update(&mut tx).await?;
    tx.commit().await?;

    jar.remove(Cookie::named(ACCESS_TOKEN));

    Ok(())
}

#[get("/signout")]
async fn signout(_user: AuthUser, jar: &CookieJar<'_>) -> Result<(), Error> {
    jar.remove(Cookie::named(ACCESS_TOKEN));

    Ok(())
}
