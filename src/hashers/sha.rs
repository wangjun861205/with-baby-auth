use crate::errors::Error;
use crate::{core::Hasher, errors};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::ThreadRng,
};
use sha2::{Digest, Sha384};
use std::io::Write;

pub struct SHA384Hasher;

impl Hasher for &SHA384Hasher {
    fn gen_salt(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>>>>
    {
        Box::pin(async move {
            let mut rng = ThreadRng::default();
            Ok(Alphanumeric.sample_string(&mut rng, 32))
        })
    }

    fn hash(
        self,
        origin: &str,
        salt: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, crate::errors::Error>>>>
    {
        let origin = origin.to_owned();
        let salt = salt.to_owned();
        Box::pin(async move {
            let mut encoder = Sha384::new();
            encoder
                .write(format!("{}{}", origin, salt).as_bytes())
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_HASH_PASSWORD))?;
            Ok(hex::encode(encoder.finalize()))
        })
    }
}
