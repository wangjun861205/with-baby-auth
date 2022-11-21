use crate::core::Storer;
use crate::errors::{self, Error};
use crate::models::Account;
use mongodb::bson::{oid::ObjectId, Bson};
use serde::Deserialize;
use std::sync::Arc;

use mongodb::{
    bson::{self, doc},
    options::ClientOptions,
    Client,
};

#[derive(Debug, Clone)]
pub struct MongoStorer {
    client: Arc<Client>,
}

#[derive(Debug, Deserialize)]
struct Acct {
    #[serde(rename(deserialize = "_id"))]
    id: ObjectId,
    username: String,
    password: String,
    salt: String,
}

impl MongoStorer {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let options = ClientOptions::parse(uri)
            .await
            .map_err(|e| Error::new(&e.to_string(), errors::INVALID_DATABASE_URI))?;
        Ok(Self {
            client: Arc::new(
                Client::with_options(options)
                    .map_err(|e| Error::new(&e.to_string(), errors::INVALID_DATABASE_OPTIONS))?,
            ),
        })
    }
}

impl Storer<String> for &MongoStorer {
    fn exists(
        self,
        name: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, Error>>>> {
        let client = self.client.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let count = client
                .database("with-baby-auth")
                .collection::<Bson>("users")
                .count_documents(doc! {"username": name}, None)
                .await
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            Ok(count > 0)
        })
    }

    fn get(
        self,
        name: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Account, Error>>>> {
        let client = self.client.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let res: Option<Bson> = client
                .database("with-baby-auth")
                .collection("users")
                .find_one(doc! {"username": name}, None)
                .await
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            if let Some(u) = res {
                let acct: Acct =
                    bson::from_bson(u).map_err(|e| Error::new(&format!("{}", e), 500))?;
                return Ok(Account {
                    id: acct.id.to_hex(),
                    username: acct.username,
                    password: acct.password,
                    salt: acct.salt,
                });
            }
            Err(Error::new("account not exists", errors::ACCOUNT_NOT_EXISTS))
        })
    }

    fn insert(
        self,
        name: &str,
        pwd: &str,
        slt: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, Error>>>> {
        let client = self.client.clone();
        let name = name.to_owned();
        let pwd = pwd.to_owned();
        let slt = slt.to_owned();
        Box::pin(async move {
            let inserted_id = client
                .database("with-baby-auth")
                .collection("users")
                .insert_one(
                    crate::models::AccountInsertion {
                        username: name.to_owned(),
                        password: pwd.to_owned(),
                        salt: slt.to_owned(),
                    },
                    None,
                )
                .await
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_INSERT_ACCOUNT))?
                .inserted_id
                .as_object_id()
                .unwrap()
                .to_hex();
            Ok(inserted_id)
        })
    }
}

#[cfg(test)]
mod test {
    use super::MongoStorer;
    use crate::core::Storer;

    #[tokio::test]
    async fn test_insert() {
        let s = &MongoStorer::new("mongodb://localhost:27017").await.unwrap();
        let id = s.insert("tongyao", "password", "salt").await.unwrap();
        println!("{:?}", id)
    }
}
