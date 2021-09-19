use super::{app_state::AppState, error::Error, token::Token};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Default)]
pub struct AuthUser;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token_str) = req.cookies().get("token") {
            if let Some(state) = req.rocket().state::<AppState>() {
                if let Ok(secret) = state.get_secret() {
                    if let Ok(token) = Token::decode(token_str.value(), &secret) {
                        if token.uid > 0 && token.permission > 0 {
                            return Outcome::Success(AuthUser::default());
                        } else {
                            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
                        }
                    }
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
    }
}
