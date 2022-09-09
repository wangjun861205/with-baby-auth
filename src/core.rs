use super::errors::{Error, ACCOUNT_ALREADY_EXISTS, INVALID_CREDENTIAL};
use super::models::Account;
use std::future::Future;
use std::pin::Pin;

pub trait Tokener {
    fn gen<'a>(&'a self, id: i32) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'a>>;
    fn verify<'a>(
        &'a self,
        token: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + 'a>>;
}

pub trait Storer {
    fn exists<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, Error>> + 'a>>;
    fn insert<'a>(
        &'a self,
        username: &'a str,
        password: &'a str,
        salt: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + 'a>>;
    fn get<'a>(
        &'a self,
        username: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Account, Error>> + 'a>>;
}

pub trait Hasher {
    fn gen_salt(&self) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn hash(
        &self,
        origin: &str,
        salt: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
}

pub fn signup<'a, S: Storer + 'a, H: Hasher + 'a>(
    username: &'a str,
    password: &'a str,
    storer: S,
    hasher: H,
) -> Pin<Box<dyn Future<Output = Result<i32, Error>> + 'a>> {
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

pub fn signin<'a, S: Storer + 'a, H: Hasher + 'a, T: Tokener + 'a>(
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
