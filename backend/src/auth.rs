use crate::entity::AppState;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;

pub struct AppFirstRun {
    pub first_run: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AppFirstRun {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let state = match req.rocket().state::<AppState>() {
            Some(state) => state,
            None => return error500,
        };

        let first_run = match state.first_run.lock() {
            Ok(v) => *v,
            Err(_) => return error500,
        };

        match first_run {
            true => Outcome::Success(AppFirstRun { first_run: true }),
            false => Outcome::Forward(()),
        }
    }
}

pub struct AppDb {
    pub conn: PoolConnection<Sqlite>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AppDb {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let state = match req.rocket().state::<AppState>() {
            Some(state) => state,
            None => return error500,
        };

        match state.pool.acquire().await {
            Ok(conn) => Outcome::Success(AppDb { conn }),
            Err(_) => error500,
        }
    }
}
