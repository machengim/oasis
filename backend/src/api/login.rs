use crate::entity::app_state::Db;
use crate::entity::request::LoginRequest;
use crate::entity::token::Claim;
use crate::entity::user;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;

pub fn route() -> Vec<Route> {
    routes![login]
}

// TODO: set token to cookie.
#[post("/login", data = "<login_req>")]
async fn login(login_req: Json<LoginRequest>, mut db: Db) -> Result<(), Status> {
    let user = match user::login_user(&login_req.username, &login_req.password, &mut db.conn).await
    {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Got error when user logins in: {}", e);
            return Err(Status::NotFound);
        }
    };

    let claim = Claim::new(user.user_id, user.permission);

    Ok(())
}
