use crate::schema::*;
use diesel::prelude::*;

use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = accounts)]
pub struct AccountInsertion {
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Clone, Insertable, Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
}
