use crate::schema::*;
use serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = accounts)]
pub struct AccountInsertion {
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: String,
    pub salt: String,
}
