use async_trait::async_trait;
use core_lib::daos::DAO;
use futures::TryStreamExt;
use log::error;
use mongodb::Collection;
use paq1_lib_error_handler::prelude::{ErrorWithCodeBuilder, ResultErr};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::daos::mongo::identifier::HasIdentifier;
use crate::query::DocumentQuery;
use core_lib::query::Query;
use mongodb::bson::doc;

pub struct MongoDao<DBO>
where
    DBO: Send + Sync + HasIdentifier,
{
    pub collection: Collection<DBO>
}

impl<DBO> MongoDao<DBO>
where
    DBO: DeserializeOwned + Send + Sync + HasIdentifier,
{
    async fn find_all(&self, query: &Query) -> Result<Vec<DBO>, mongodb::error::Error> {
        let document_wrapper: DocumentQuery = query.clone().into();
        self.collection
            .find(document_wrapper.filter)
            .await?
            .try_collect::<Vec<DBO>>()
            .await
            .map_err(|error| {
                error!("mongodb error: {error:?}");
                error
            })
    }
}

#[async_trait]
impl<DBO> DAO<DBO, String> for MongoDao<DBO>
where
    DBO: Serialize + DeserializeOwned + Send + Sync + HasIdentifier,
{
    async fn fetch_one(&self, id: &String) -> ResultErr<Option<DBO>> {
        let filter = doc! {"id": id};
        self.collection
            .find_one(filter)
            .await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONFO", 500, err.to_string().as_str()).build()
            })
    }

    async fn fetch_all(&self, query: &Query) -> ResultErr<Vec<DBO>> {
        self.find_all(query).await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONFA", 500, err.to_string().as_str())
                    .build()
            })
    }

    async fn insert(&self, entity: &DBO) -> ResultErr<String> {
        let id = entity.identifier();
        self.collection
            .insert_one(entity)
            .await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONIN", 500, err.to_string().as_str())
                    .build()
            })
            .map(|_| id.clone())
    }

    async fn update(&self, entity: &DBO) -> ResultErr<String> {
        let id = entity.identifier();
        let filter = doc! { "id": id };
        self.collection
            .replace_one(filter, entity)
            .await
            .map(|_| id.clone())
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONUP", 500, err.to_string().as_str())
                    .build()
            })
    }

    async fn delete(&self, id: &String) -> ResultErr<()> {
        self.collection
            .delete_one(doc! { "id": id })
            .await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONDL", 500, err.to_string().as_str())
                    .build()
            })
            .map(|_| ())
    }
}
