use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::pool::PoolConnection;
use sqlx::Pool;
use sqlx::Sqlite;
use std::sync::Mutex;

pub struct AppState {
    pub first_run: Mutex<bool>,
    pub pool: Pool<Sqlite>,
    pub storage: Mutex<String>,
    pub secret: Mutex<String>,
}

pub struct FirstRun {
    pub first_run: bool,
}

pub struct Db {
    pub conn: PoolConnection<Sqlite>,
}

pub struct Secret {
    pub key: String,
}

pub struct Storage {
    pub path: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FirstRun {
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
            true => Outcome::Success(FirstRun { first_run: true }),
            false => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Db {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let state = match req.rocket().state::<AppState>() {
            Some(state) => state,
            None => return error500,
        };

        match state.pool.acquire().await {
            Ok(conn) => Outcome::Success(Db { conn }),
            Err(_) => error500,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Secret {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let key = match get_site_secret(req) {
            Ok(v) => v,
            Err(_) => return error500,
        };

        Outcome::Success(Secret { key })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Storage {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let state = match req.rocket().state::<AppState>() {
            Some(state) => state,
            None => return error500,
        };

        let path = match state.storage.lock() {
            Ok(v) => (*v).clone(),
            Err(_) => return error500,
        };

        Outcome::Success(Storage { path })
    }
}

pub fn get_site_secret<'r>(req: &'r Request<'_>) -> anyhow::Result<String> {
    let err = Err(anyhow::anyhow!("Cannot read site secret key"));
    let state = match req.rocket().state::<AppState>() {
        Some(state) => state,
        None => return err,
    };

    let key = match state.secret.lock() {
        Ok(v) => (*v).clone(),
        Err(_) => return err,
    };

    Ok(key)
}
