use crate::entity::{AppState, AuthDb, AuthIndex};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

// The URL redirect relies on three parts: first_run, request path, and the user role.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthIndex {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        let first_run = state.first_run.lock().unwrap();
        let path = req.uri().path().to_string();

        if path.starts_with("/api/sys/dirs") && *first_run {
            return Outcome::Success(AuthIndex { option: 1 });
        }

        match (&path[..], *first_run) {
            ("/", false) | ("/index.html", false) => Outcome::Forward(()),
            ("/", true) | ("/index.html", true) => Outcome::Success(AuthIndex { option: 1 }),
            ("/setup", true) => Outcome::Success(AuthIndex { option: 1 }),
            ("/api/sys/volumes", true) => Outcome::Success(AuthIndex { option: 1 }),
            ("/api/setup", true) => Outcome::Success(AuthIndex { option: 1 }),
            _ => Outcome::Failure((Status::Forbidden, Status::Forbidden)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthDb {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Status> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));
        let error403 = Outcome::Failure((Status::Forbidden, Status::Forbidden));

        let state = match req.rocket().state::<AppState>() {
            Some(state) => state,
            None => return error500,
        };

        let first_run = match state.first_run.lock() {
            Ok(v) => *v,
            Err(_) => return error500,
        };

        if first_run {
            Outcome::Success(AuthDb {
                conn: match state.pool.acquire().await {
                    Ok(conn) => conn,
                    Err(_) => return error500,
                },
            })
        } else {
            error403
        }
    }
}
