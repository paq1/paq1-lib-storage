use async_trait::async_trait;
use core_lib::daos::DAO;
use futures::TryStreamExt;
use mongodb::Collection;
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
    }
}

#[async_trait]
impl<DBO> DAO<DBO, String, mongodb::error::Error> for MongoDao<DBO>
where
    DBO: Serialize + DeserializeOwned + Send + Sync + HasIdentifier,
{
    async fn fetch_one(&self, id: &String) -> Result<Option<DBO>, mongodb::error::Error> {
        let key = DBO::identifier_key();
        let filter = doc! {key: id};
        self.collection
            .find_one(filter)
            .await
    }

    async fn fetch_all(&self, query: &Query) -> Result<Vec<DBO>, mongodb::error::Error> {
        self.find_all(query).await
    }

    async fn insert(&self, entity: &DBO) -> Result<String, mongodb::error::Error> {
        let id = entity.identifier_value();
        self.collection
            .insert_one(entity)
            .await
            .map(|_| id.clone())
    }

    async fn update(&self, entity: &DBO) -> Result<String, mongodb::error::Error> {
        let id = entity.identifier_value();
        let key = DBO::identifier_key();
        let filter = doc! { key: id };
        self.collection
            .replace_one(filter, entity)
            .await
            .map(|_| id.clone())
    }

    async fn delete(&self, id: &String) -> Result<(), mongodb::error::Error> {
        let key = DBO::identifier_key();
        self.collection
            .delete_one(doc! { key: id })
            .await
            .map(|_| ())
    }

    async fn delete_all(&self) -> Result<(), mongodb::error::Error> {
        self.collection
            .delete_many(doc! {})
            .await.map(|_| ())
    }
}
