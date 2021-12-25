use super::{app_state::AppState, token::AccessToken, token::Token};
use crate::entity::error::Error;
use crate::util::constants::ACCESS_TOKEN;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Default)]
pub struct AuthUser {
    pub uid: i64,
    pub permission: i8,
}

#[derive(Default)]
pub struct AuthAdmin {
    pub uid: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token_str) = req.cookies().get(ACCESS_TOKEN) {
            if let Some(state) = req.rocket().state::<AppState>() {
                if let Ok(secret) = state.get_secret() {
                    if let Ok(token) = AccessToken::decode(token_str.value(), &secret) {
                        if let Ok(allow_guest) = state.get_allow_guest() {
                            if token.uid > (0 - allow_guest) as i64
                                && token.permission > 0 - allow_guest
                            {
                                return Outcome::Success(AuthUser {
                                    uid: token.uid,
                                    permission: token.permission,
                                });
                            } else {
                                return Outcome::Failure((
                                    Status::Unauthorized,
                                    Error::Unauthorized,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAdmin {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token_str) = req.cookies().get(ACCESS_TOKEN) {
            if let Some(state) = req.rocket().state::<AppState>() {
                if let Ok(secret) = state.get_secret() {
                    if let Ok(token) = AccessToken::decode(token_str.value(), &secret) {
                        if token.uid > 0 && token.permission == 9 {
                            return Outcome::Success(AuthAdmin { uid: token.uid });
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
