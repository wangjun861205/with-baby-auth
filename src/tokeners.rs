use super::core::Tokener;
use super::errors::{self, Error};
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
    fn gen<'a>(
        &'a self,
        id: i32,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>> + 'a>,
    > {
        Box::pin(async move {
            UID(id)
                .sign_with_key(&self.key)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_SIGN_CLAIM))
        })
    }

    fn verify<'a>(
        &'a self,
        token: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<i32, Error>> + 'a>> {
        Box::pin(async move {
            let uid: UID = token
                .verify_with_key(&self.key)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_VERIFY_TOKEN))?;
            Ok(uid.0)
        })
    }
}
