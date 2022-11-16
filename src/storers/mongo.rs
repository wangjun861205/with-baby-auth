use crate::core::Storer;
use crate::errors::{self, Error};
use mongodb::bson::Bson;

use mongodb::{
    bson::{self, doc},
    options::ClientOptions,
    Client,
};

#[derive(Debug, Clone)]
pub struct MongoStorer {
    client: Client,
}

impl MongoStorer {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let options = ClientOptions::parse(uri)
            .await
            .map_err(|e| Error::new(&e.to_string(), errors::INVALID_DATABASE_URI))?;
        Ok(Self {
            client: Client::with_options(options)
                .map_err(|e| Error::new(&e.to_string(), errors::INVALID_DATABASE_OPTIONS))?,
        })
    }
}

impl Storer<String> for &MongoStorer {
    fn exists<'a>(
        &'a self,
        name: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, Error>> + 'a>> {
        Box::pin(async move {
            let count = self
                .client
                .database("with-baby-auth")
                .collection::<Bson>("users")
                .count_documents(doc! {"username": name}, None)
                .await
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            Ok(count > 0)
        })
    }

    fn get<'a>(
        &'a self,
        name: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<crate::models::Account, Error>> + 'a>,
    > {
        Box::pin(async move {
            let res: Option<Bson> = self
                .client
                .database("with-baby-auth")
                .collection("users")
                .find_one(doc! {"username": name}, None)
                .await
                .map_err(|e| Error::new(&e.to_string(), errors::FAILED_TO_LOAD_RECORD))?;
            if let Some(u) = res {
                return Ok(bson::from_bson(u).unwrap());
            }
            Err(Error::new("account not exists", errors::ACCOUNT_NOT_EXISTS))
        })
    }

    fn insert<'a>(
        &'a self,
        name: &'a str,
        pwd: &'a str,
        slt: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, Error>> + 'a>> {
        Box::pin(async move {
            let inserted_id = self
                .client
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
