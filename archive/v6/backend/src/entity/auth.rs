use super::app;
use super::token::Claim;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct AuthUser {
    pub claim: Claim,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error500 = Outcome::Failure((Status::InternalServerError, Status::InternalServerError));

        let token = match req.cookies().get("token") {
            Some(v) => v.value(),
            None => return Outcome::Forward(()),
        };

        let secret = match app::get_site_secret(req) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                return error500;
            }
        };

        // Validation error needs forward.
        let claim = match Claim::from(token, &secret) {
            Ok(v) => v,
            Err(_) => return Outcome::Forward(()),
        };

        if claim.permission > 0 {
            return Outcome::Success(AuthUser { claim });
        }

        Outcome::Forward(())
    }
}
