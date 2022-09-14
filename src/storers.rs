use crate::core::Storer;
use crate::diesel::{
    dsl::{exists, select},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use crate::errors::{self, Error};
use crate::schema::accounts::dsl::*;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use crate::models::AccountInsertion;

#[derive(Debug, Clone)]
pub struct PgStorer {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgStorer {
    pub fn new(url: &str) -> Result<Self, Error> {
        let mgr = ConnectionManager::new(url);
        let pool = Pool::new(mgr)
            .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_CONNECT_TO_DATABASE))?;
        Ok(Self { pool })
    }
}

impl Storer for &PgStorer {
    fn exists<'a>(
        &'a self,
        name: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, Error>> + 'a>> {
        Box::pin(async move {
            let mut conn = self.pool.get().map_err(|e| {
                Error::new(&e.to_string(), errors::FAILED_TO_GET_DATABASE_CONNECTION)
            })?;
            let res: bool = select(exists(accounts.filter(username.eq(name))))
                .get_result(&mut conn)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            Ok(res)
        })
    }

    fn get<'a>(
        &'a self,
        name: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<crate::models::Account, Error>> + 'a>,
    > {
        Box::pin(async move {
            let mut conn = self.pool.get().map_err(|e| {
                Error::new(&e.to_string(), errors::FAILED_TO_GET_DATABASE_CONNECTION)
            })?;
            accounts
                .filter(username.eq(name))
                .get_result(&mut conn)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))
        })
    }

    fn insert<'a>(
        &'a self,
        name: &'a str,
        pwd: &'a str,
        slt: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<i32, Error>> + 'a>> {
        Box::pin(async move {
            let mut conn = self.pool.get().map_err(|e| {
                Error::new(&e.to_string(), errors::FAILED_TO_GET_DATABASE_CONNECTION)
            })?;
            diesel::insert_into(accounts)
                .values(AccountInsertion {
                    username: name.to_owned(),
                    password: pwd.to_owned(),
                    salt: slt.to_owned(),
                })
                .returning(id)
                .get_result(&mut conn)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))
        })
    }
}
