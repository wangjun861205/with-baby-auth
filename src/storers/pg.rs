use crate::core::Storer;
use crate::diesel::{
    dsl::{exists, select},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use crate::errors::{self, Error};
use crate::models::Account;
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

impl Storer<i32> for &PgStorer {
    fn exists(
        self,
        name: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, Error>>>> {
        let pool = self.pool.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let mut conn = pool.get().map_err(|e| {
                Error::new(&e.to_string(), errors::FAILED_TO_GET_DATABASE_CONNECTION)
            })?;
            let res: bool = select(exists(accounts.filter(username.eq(name))))
                .get_result(&mut conn)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            Ok(res)
        })
    }

    fn get(
        self,
        name: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Account, Error>>>> {
        let pool = self.pool.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let mut conn = pool.get().map_err(|e| {
                Error::new(&e.to_string(), errors::FAILED_TO_GET_DATABASE_CONNECTION)
            })?;
            accounts
                .filter(username.eq(name))
                .get_result::<(i32, String, String, String)>(&mut conn)
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))
                .map(|(_id, _username, _password, _salt)| Account {
                    id: _id.to_string(),
                    username: _username,
                    password: _password,
                    salt: _salt,
                })
        })
    }

    fn insert(
        self,
        name: &str,
        pwd: &str,
        slt: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<i32, Error>>>> {
        let pool = self.pool.clone();
        let name = name.to_owned();
        let pwd = pwd.to_owned();
        let slt = slt.to_owned();
        Box::pin(async move {
            let mut conn = pool.get().map_err(|e| {
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
