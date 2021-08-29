use super::state::State;
use crate::util::env;
use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tide::Request;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub exp: usize,
    pub uid: i64,
    pub username: String,
    pub permission: i16,
}

impl Token {
    pub fn new(uid: i64, username: &str, permission: i16) -> Self {
        let token_expire_days: i64 = env::must_get_env_value("TOKEN_EXPIRE_DAYS", 7);
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

    pub fn encode(&self, secret: &str) -> Result<String> {
        let token_string = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token_string)
    }

    fn decode(token: &str, secret: &str) -> Result<Token> {
        let token = decode::<Token>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }

    pub fn from_cookie(req: &Request<State>) -> Result<Token> {
        let token_in_cookie = match req.cookie("token") {
            Some(v) => v.value().to_owned(),
            None => return Err(anyhow!("No token found in cookie")),
        };

        let secret = req.state().get_secret()?;
        let token = Self::decode(&token_in_cookie, &secret)?;

        Ok(token)
    }

    pub fn from_ext(req: &Request<State>) -> Result<Self> {
        match req.ext::<Self>() {
            Some(token) => Ok(token.clone()),
            None => Err(anyhow!("No token found")),
        }
    }

    pub fn auth_user_permission(req: &Request<State>) -> i16 {
        match req.ext::<Self>() {
            Some(v) => v.permission,
            _ => -1,
        }
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
