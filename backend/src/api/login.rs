use crate::db;
use crate::entity::auth::{Db, FirstRun};
use crate::entity::request::LoginRequest;
use crate::entity::site::{self, AppState};
use crate::entity::user;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn route() -> Vec<Route> {
    routes![login]
}

// TODO: not finished yet.
#[post("/login", data = "<login_req>")]
async fn login(login_req: Json<LoginRequest>, mut db: Db) -> Result<(), Status> {
    let error500 = Err(Status::InternalServerError);

    let user = match user::login_user(&login_req.username, &login_req.password, &mut db.conn).await
    {
        Ok(u) => u,
        Err(e) => return error500,
    };

    Ok(())
}
