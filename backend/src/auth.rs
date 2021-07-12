use crate::entity::{AppState, Auth};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

// The URL redirect relies on three parts: first_run, request path, and the user role.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        let first_run = state.first_run.lock().unwrap();
        let path = req.uri().path().to_string();

        if path.starts_with("/api/sys/dirs") && *first_run {
            return Outcome::Success(Auth { option: 1 });
        }

        match (&path[..], *first_run) {
            ("/", false) | ("/index.html", false) => Outcome::Forward(()),
            ("/", true) | ("/index.html", true) => Outcome::Success(Auth { option: 1 }),
            ("/setup", true) => Outcome::Success(Auth { option: 1 }),
            ("/api/sys/volumes", true) => Outcome::Success(Auth { option: 1 }),
            ("/api/setup", true) => Outcome::Success(Auth { option: 1 }),
            _ => Outcome::Failure((Status::Forbidden, Status::Forbidden)),
        }
    }
}
