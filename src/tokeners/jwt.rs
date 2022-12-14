use crate::core::Tokener;
use crate::errors::{self, Error};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;

#[derive(Debug, Clone)]
pub struct JWTTokener {
    key: Hmac<Sha384>,
}

impl JWTTokener {
    pub fn new(key: &str) -> Result<Self, Error> {
        Ok(Self {
            key: Hmac::new_from_slice(key.as_bytes())
                .map_err(|_| Error::new("invalid tokener key", errors::INVALID_TOKENER_KEY))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UID(i32);

impl Tokener for &JWTTokener {
    fn gen(
        self,
        id: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>>>>
    {
        let key = self.key.clone();
        let id = id.to_owned();
        Box::pin(async move {
            id.sign_with_key(&key)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_SIGN_CLAIM))
        })
    }

    fn verify(
        self,
        token: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<i32, Error>>>> {
        let key = self.key.clone();
        let token = token.to_owned();
        Box::pin(async move {
            let uid: UID = token
                .verify_with_key(&key)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_VERIFY_TOKEN))?;
            Ok(uid.0)
        })
    }
}
