use crate::core;
use crate::errors::{self, Error};
use crate::hashers::SHA384Hasher;
use crate::storers::mongo::MongoStorer;
use crate::tokeners::JWTTokener;
use actix_header::actix_header;
use actix_web::body::BoxBody;
use actix_web::{
    http::StatusCode,
    web::{Data, Header, Json, Query},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if self.code == errors::INVALID_CREDENTIAL || self.code == errors::FAILED_TO_VERIFY_TOKEN {
            return HttpResponse::with_body(
                StatusCode::FORBIDDEN,
                BoxBody::new(self.msg.to_owned()),
            );
        }
        HttpResponse::with_body(
            StatusCode::INTERNAL_SERVER_ERROR,
            BoxBody::new(self.msg.to_owned()),
        )
    }
}

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
    Json(data): Json<SigninRequest>,
) -> Result<HttpResponse, Error> {
    let token = core::signin(
        &data.username,
        &&data.password,
        storer.as_ref(),
        hasher.as_ref(),
        tokener.as_ref(),
    )
    .await?;
    Ok(HttpResponse::Ok()
        .append_header(("X-JWT-TOKEN", token))
        .finish())
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

pub async fn verify(
    tokener: Data<JWTTokener>,
    Header(TokenHeader(token)): Header<TokenHeader>,
) -> Result<HttpResponse, Error> {
    let uid = core::verify_token(&token, tokener.as_ref()).await?;
    Ok(HttpResponse::Ok().append_header(("X-UID", uid)).finish())
}

#[derive(Debug, Deserialize)]
pub struct ExistsRequest {
    username: String,
}

#[derive(Debug, Serialize)]
pub struct ExistsResponse {
    exists: bool,
}

pub async fn exists(
    storer: Data<MongoStorer>,
    Query(ExistsRequest { username }): Query<ExistsRequest>,
) -> Result<Json<ExistsResponse>, Error> {
    Ok(Json(ExistsResponse {
        exists: core::exists(&username, storer.as_ref()).await?,
    }))
}
