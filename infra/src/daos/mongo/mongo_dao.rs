use async_trait::async_trait;
use core_lib::daos::DAO;
use log::{error, info};
use futures::TryStreamExt;
use mongodb::Collection;
use paq1_lib_error_handler::prelude::{ErrorWithCodeBuilder, ResultErr};
use serde::de::DeserializeOwned;
use serde::Serialize;

use mongodb::bson::doc;
use paq1_lib_error_handler::prelude::Error::ErrorWithCode;
use core_lib::query::Query;
use crate::query::DocumentWrapper;

pub struct MongoDao<DBO>
where
    DBO: Send + Sync,
{
    pub collection: Collection<DBO>
}

impl<DBO> MongoDao<DBO>
where
    DBO: DeserializeOwned + Send + Sync,
{
    async fn find_all(&self, query: &Query) -> Result<Vec<DBO>, mongodb::error::Error> {
        let document_wrapper: DocumentWrapper = query.clone().into();
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
    DBO: Serialize + DeserializeOwned + Send + Sync,
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
        // if !self.is_connected().await {
        //     error!("la connexion au client mongo est perdu");
        // } else {
        //     info!("la connexion est ok");
        // }

        self.find_all(query).await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONFA", 500, err.to_string().as_str())
                    .build()
            })
    }

    async fn insert(&self, entity: &DBO, entity_id: &String) -> ResultErr<String> {
        todo!()
    }

    async fn update(&self, id: &String, entity: &DBO) -> ResultErr<String> {
        todo!()
    }
}
