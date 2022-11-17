use crate::core;
use crate::errors::{self, Error};
use crate::hashers::sha::SHA384Hasher;
use crate::storers::mongo::MongoStorer;
use crate::tokeners::jwt::JWTTokener;
use actix_header::actix_header;
use actix_web::body::BoxBody;
use actix_web::{
    http::StatusCode,
    web::{Data, Header, Json, Path, Query},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
}

pub async fn signup(
    hasher: Data<SHA384Hasher>,
    storer: Data<MongoStorer>,
    Json(data): Json<SignupRequest>,
) -> Result<String, Error> {
    let id = core::signup(
        &data.username,
        &data.password,
        storer.as_ref(),
        hasher.as_ref(),
    )
    .await?;
    Ok(id.to_string())
}

#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    username: String,
    password: String,
}

pub async fn signin(
    hasher: Data<SHA384Hasher>,
    storer: Data<MongoStorer>,
    tokener: Data<JWTTokener>,
    Query(SigninRequest { username, password }): Query<SigninRequest>,
) -> Result<String, Error> {
    let token = core::signin(
        &username,
        &password,
        storer.as_ref(),
        hasher.as_ref(),
        tokener.as_ref(),
    )
    .await?;
    Ok(token)
}

#[actix_header("X-JWT-TOKEN")]
pub struct TokenHeader(String);

impl From<String> for TokenHeader {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<TokenHeader> for String {
    fn from(t: TokenHeader) -> Self {
        t.0
    }
}

pub async fn verify_token(
    tokener: Data<JWTTokener>,
    token: Path<(String,)>,
) -> Result<String, Error> {
    let uid = core::verify_token(&token.0, tokener.as_ref()).await?;
    Ok(uid.to_string())
}

#[derive(Debug, Deserialize)]
pub struct ExistsRequest {
    username: String,
}

#[derive(Debug, Serialize)]
pub struct ExistsResponse {
    exists: bool,
}

pub async fn validate_username(
    storer: Data<MongoStorer>,
    username: Path<(String,)>,
) -> Result<String, Error> {
    if core::exists(&username.0, storer.as_ref()).await? {
        return Err(Error::new("username already exists", 409));
    }
    Ok("ok".into())
}
