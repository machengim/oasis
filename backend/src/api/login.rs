use crate::entity::app_state::{Db, Secret};
use crate::entity::request::LoginRequest;
use crate::entity::token::Claim;
use crate::entity::user;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::Route;

pub fn route() -> Vec<Route> {
    routes![login]
}

// TODO: set token to cookie.
#[post("/login", data = "<login_req>")]
async fn login(
    login_req: Json<LoginRequest>,
    mut db: Db,
    secret: Secret,
    jar: &CookieJar<'_>,
) -> Result<(), Status> {
    let user = match user::login_user(&login_req.username, &login_req.password, &mut db.conn).await
    {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Got error when user logins in: {}", e);
            return Err(Status::NotFound);
        }
    };

    let claim = Claim::new(user.user_id, user.permission);
    let token = match claim.to_token(&secret.key) {
        Ok(v) => v,
        Err(_) => return Err(Status::InternalServerError),
    };

    let cookie = Cookie::build("token", token)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(7));
    jar.add(cookie.finish());

    Ok(())
}
