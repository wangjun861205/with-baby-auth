use super::errors::{Error, ACCOUNT_ALREADY_EXISTS, INVALID_CREDENTIAL};
use super::models::Account;
use std::future::Future;
use std::pin::Pin;

pub trait Tokener {
    fn gen(self, id: i32) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn verify(self, token: &str) -> Pin<Box<dyn Future<Output = Result<i32, Error>>>>;
}

pub trait Storer<T> {
    fn exists(self, username: &str) -> Pin<Box<dyn Future<Output = Result<bool, Error>>>>;
    fn insert(
        self,
        username: &str,
        password: &str,
        salt: &str,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;
    fn get(self, username: &str) -> Pin<Box<dyn Future<Output = Result<Account, Error>>>>;
}

pub trait Hasher {
    fn gen_salt(self) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn hash(self, origin: &str, salt: &str)
        -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
}

pub fn signup<'a, ST, S: Storer<ST> + Copy + 'a, H: Hasher + Copy + 'a>(
    username: &'a str,
    password: &'a str,
    storer: S,
    hasher: H,
) -> Pin<Box<dyn Future<Output = Result<ST, Error>> + 'a>> {
    Box::pin(async move {
        let is_exists = storer.exists(username).await?;
        if is_exists {
            return Err(Error::new("account already exists", ACCOUNT_ALREADY_EXISTS));
        }
        let salt = hasher.gen_salt().await?;
        let hashed_password = hasher.hash(password, &salt).await?;
        storer.insert(username, &hashed_password, &salt).await
    })
}

pub fn signin<'a, ST, S: Storer<ST> + 'a, H: Hasher + 'a, T: Tokener + 'a>(
    username: &'a str,
    password: &'a str,
    storer: S,
    hasher: H,
    tokener: T,
) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'a>> {
    Box::pin(async move {
        let account = storer.get(username).await?;
        let hashed_password = hasher.hash(password, &account.salt).await?;
        if hashed_password != account.password {
            return Err(Error::new("invalid credential", INVALID_CREDENTIAL));
        }
        tokener.gen(account.id).await
    })
}

pub fn verify_token<'a, T: Tokener + 'a>(
    token: &'a str,
    tokener: T,
) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + 'a>> {
    Box::pin(async move { tokener.verify(token).await })
}

pub fn exists<'a, ST, S: Storer<ST> + 'a>(
    username: &'a str,
    storer: S,
) -> Pin<Box<dyn Future<Output = Result<bool, Error>> + 'a>> {
    Box::pin(async move { storer.exists(username).await })
}
