use crate::core::Hasher;
use rand::{rngs::ThreadRng, Rng};
use sha2::{Digest, Sha384};

pub struct SHA384Hasher;

impl Hasher for SHA384Hasher {
    fn gen_salt(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>>>>
    {
        Box::pin(async move {
            let rng = ThreadRng::default();

            let mut arr = [0i8; 32];
            rng.fill(&mut arr);
            Ok(base64::encode(
                &arr.into_iter().map(|v| v as u8).collect::<Vec<u8>>(),
            ))
        })
    }

    fn hash(
        &self,
        origin: &str,
        salt: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>>>>
    {
        Box::pin(async move {})
    }
}
