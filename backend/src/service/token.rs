use super::{app_state::AppState, error::Error};
use anyhow::Result as AnyResult;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub exp: usize,
    pub uid: i64,
    pub username: String,
    pub permission: i8,
}

impl Token {
    pub fn new(uid: i64, username: &str, permission: i8) -> Self {
        let token_expire_days: i64 = 7;
        let expire_time = chrono::Utc::now().timestamp() + token_expire_days * 24 * 60 * 60;

        Token {
            exp: expire_time as usize,
            uid,
            username: username.to_string(),
            permission,
        }
    }

    pub fn default() -> Self {
        Token::new(-1, "", -1)
    }

    pub fn encode(&self, secret: &str) -> AnyResult<String> {
        let token_string = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token_string)
    }

    pub fn decode(token: &str, secret: &str) -> AnyResult<Token> {
        let token = decode::<Token>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token_str) = req.cookies().get("token") {
            if let Some(state) = req.rocket().state::<AppState>() {
                if let Ok(secret) = state.get_secret() {
                    if let Ok(token) = Token::decode(token_str.value(), &secret) {
                        return Outcome::Success(token);
                    }
                }
            }
        }

        Outcome::Success(Token::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_should_work() {
        let secret = "mySitePassword";
        let claim = Token::new(1, "", 9);
        let token = claim.encode(secret).unwrap();
        let validate = Token::decode(&token, secret).unwrap();
        assert_eq!(validate.permission, 9);
    }

    #[test]
    #[should_panic]
    fn test_token_should_panic() {
        let mut claim = Token::new(1, "", 9);
        claim.exp -= 7 as usize * 24 * 60 * 60 + 1;
        let token = claim.encode("secret").unwrap();
        let validate = Token::decode(&token, "secret").unwrap();
        assert_eq!(validate.permission, 9);
    }
}
