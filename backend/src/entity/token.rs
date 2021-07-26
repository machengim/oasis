use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
const EXPIRE_DAYS: i64 = 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub exp: usize,
    pub uid: i64,
    pub permission: u8,
}

impl Claim {
    pub fn new(uid: i64, permission: u8) -> Self {
        let expire_time = chrono::Utc::now().timestamp() + EXPIRE_DAYS * 24 * 60 * 60;
        Claim {
            exp: expire_time as usize,
            uid,
            permission,
        }
    }

    pub fn to_token(&self, secret: &str) -> anyhow::Result<String> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn from(token: &str, secret: &str) -> anyhow::Result<Claim> {
        let token = decode::<Claim>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_should_work() {
        let claim = Claim::new(1, 9);
        let token = claim.to_token("secret").unwrap();
        let validate = Claim::from(&token, "secret").unwrap();
        assert_eq!(validate.permission, 9);
    }

    #[test]
    #[should_panic]
    fn test_token_should_panic() {
        let mut claim = Claim::new(1, 9);
        claim.exp -= EXPIRE_DAYS as usize * 24 * 60 * 60 + 1;
        let token = claim.to_token("secret").unwrap();
        let validate = Claim::from(&token, "secret").unwrap();
        assert_eq!(validate.permission, 9);
    }
}
