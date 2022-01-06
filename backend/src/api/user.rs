use crate::entity::error::Error;
use crate::entity::request::{ChangePasswordRequest, ForgotPasswordRequest, LoginRequest};
use crate::entity::reset_password::ResetPassword;
use crate::entity::response::LoginResponse;
use crate::entity::user::User;
use crate::service::app_state::AppState;
use crate::service::auth::AuthUser;
use crate::service::token::{AccessToken, RefreshToken, Token};
use crate::util::{self, constants::*};
use anyhow::Result as AnyResult;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::{Route, State};
use sqlx::Connection;

pub fn route() -> Vec<Route> {
    routes![
        login,
        signout,
        change_password,
        refresh_access_token,
        guest_login,
        forgot_password
    ]
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

    let access_token = set_access_token(&user, &secret, jar)?;
    set_refresh_token(&user, &secret, jar)?;

    let login = LoginResponse {
        username: user.username,
        permission: user.permission,
        expire: access_token.exp,
    };

    Ok(Json(login))
}

#[get("/login/guest")]
async fn guest_login(
    state: &State<AppState>,
    jar: &CookieJar<'_>,
) -> Result<Json<LoginResponse>, Error> {
    let allow_guest = state.get_site()?.allow_guest;
    if allow_guest <= 0 {
        return Err(Error::BadRequest);
    }

    let current_timestamp = util::get_utc_seconds();
    let user = User::guest(current_timestamp);
    let secret = state.get_secret()?;

    let access_token = set_access_token(&user, &secret, jar)?;
    set_refresh_token(&user, &secret, jar)?;

    let login = LoginResponse {
        username: user.username,
        permission: user.permission,
        expire: access_token.exp,
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
    remove_tokens(jar);

    Ok(())
}

#[get("/user/signout")]
async fn signout(_user: AuthUser, jar: &CookieJar<'_>) -> Result<(), Error> {
    remove_tokens(jar);

    Ok(())
}

#[get("/user/refresh")]
async fn refresh_access_token(
    state: &State<AppState>,
    token: RefreshToken,
    jar: &CookieJar<'_>,
) -> Result<Json<LoginResponse>, Error> {
    let mut conn = state.get_pool_conn().await?;
    let user: User;

    if state.get_allow_guest()? > 0 && token.uid == 0 {
        user = User::guest(util::get_utc_seconds());
    } else {
        let user_op = User::find_user_by_id(token.uid, &mut conn).await?;

        if user_op.is_none() {
            return Err(Error::BadRequest);
        }
        user = user_op.unwrap();
    }

    let secret = state.get_secret()?;
    let access_token = set_access_token(&user, &secret, jar)?;
    set_refresh_token(&user, &secret, jar)?;

    let login = LoginResponse {
        username: user.username,
        permission: user.permission,
        expire: access_token.exp,
    };

    Ok(Json(login))
}

#[post("/user/forgot-password", data = "<req_body>")]
async fn forgot_password(
    state: &State<AppState>,
    req_body: Json<ForgotPasswordRequest>,
) -> Result<String, Error> {
    let mut conn = state.get_pool_conn().await?;
    let user = match User::find_user_by_name(&req_body.username, &mut conn).await? {
        Some(user) => user,
        None => return Err(Error::BadRequest),
    };

    let reset_pw = ResetPassword::new(&user.username);
    // Remove all reset password files belong to this user.
    reset_pw.remove_user_reset_password_files(&mut conn).await?;
    // Create a new reset password file.
    reset_pw.write_reset_password_file(&req_body.url).await?;

    // Write record into db.
    let mut tx = conn.begin().await?;
    reset_pw.insert_query(&mut tx).await?;
    tx.commit().await?;

    Ok(reset_pw.reset_id)
}

fn set_access_token(user: &User, secret: &str, jar: &CookieJar<'_>) -> AnyResult<AccessToken> {
    let access_token = user.generate_access_token();
    let access_token_str = access_token.encode(&secret)?;
    let cookie = Cookie::build(ACCESS_TOKEN, access_token_str)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::minutes(ACCESS_TOKEN_MINS))
        .finish();

    jar.add(cookie);

    Ok(access_token)
}

fn set_refresh_token(user: &User, secret: &str, jar: &CookieJar<'_>) -> AnyResult<()> {
    let refresh_token_str = user.generate_refresh_token().encode(&secret)?;
    let cookie = Cookie::build(REFRESH_TOKEN, refresh_token_str)
        .path("/api/user")
        .http_only(true)
        .max_age(time::Duration::days(REFRESH_TOKEN_DAYS))
        .finish();

    jar.add(cookie);

    Ok(())
}

fn remove_tokens(jar: &CookieJar<'_>) {
    jar.remove(Cookie::named(ACCESS_TOKEN));

    let cookie_refresh = Cookie::build(REFRESH_TOKEN, "")
        .path("/api/user")
        .http_only(true)
        .finish();

    jar.remove(cookie_refresh);
}
