use crate::daos::DAO;
use crate::prelude::Query;
use crate::repositories::repository::Repository;
use paq1_lib_error_handler::prelude::{ErrorWithCodeBuilder, ResultErr};
use std::sync::Arc;
use async_trait::async_trait;
use crate::data::paged::Paged;

pub struct CrudRepository<DBO, DBOERR> {
    pub dao: Arc<dyn DAO<DBO, String, DBOERR>>
}

impl<DBO, ID> CrudRepository<DBO, ID> {
    pub async fn count(&self) -> ResultErr<u64> {
        self.dao
            .count()
            .await
            .map_err(|_err| {
                ErrorWithCodeBuilder::new("00MONCO", 500, "count error").build()
            })
    }
}


#[async_trait]
impl<DATA, DBO, DBOERR> Repository<DATA, String> for CrudRepository<DBO, DBOERR>
where
    DBOERR: ToString,
    DATA: From<DBO> + Clone + Sync,
    DBO: From<DATA> + Clone + Send,
{
    async fn fetch_one(&self, id: &String) -> ResultErr<Option<DATA>> {
        self.dao.fetch_one(id).await
            .map(|data| data.map(|d| d.into()))
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONFO", 500, err.to_string().as_str()).build()
            })
    }

    async fn fetch_all(&self, query: &Query) -> ResultErr<Paged<DATA>> {
        let total_records = self.count().await? as u32;
        let page_size = query.pager.page_size;
        let total_page = ((total_records as f64) / (page_size as f64)).ceil() as u32;

        self.dao
            .fetch_all(&query)
            .await
            .map(|dbo_opt| {
                dbo_opt
                    .into_iter()
                    .map(|dbo| {
                        let data = dbo.into();
                        data
                    })
                    .collect::<Vec<_>>()
            })
            .map(|data| Paged {
                data,
                total_page,
                total_records,
                offset: query.pager.page_number,
                page_size,
            })
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONFA", 500, err.to_string().as_str()).build()
            })
    }

    async fn insert(&self, entity: &DATA) -> ResultErr<String> {
        let dbo: DBO = entity.clone().into();
        self.dao.insert(&dbo).await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONIN", 500, err.to_string().as_str()).build()
            })
    }

    async fn update(&self, entity: &DATA) -> ResultErr<String> {
        let dbo: DBO = entity.clone().into();
        self.dao.update(&dbo).await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONUP", 500, err.to_string().as_str()).build()
            })
    }

    async fn delete(&self, id: &String) -> ResultErr<()> {
        self.dao.delete(id).await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONDO", 500, err.to_string().as_str()).build()
            })
    }

    async fn delete_all(&self) -> ResultErr<()> {
        self.dao.delete_all().await
            .map_err(|err| {
                ErrorWithCodeBuilder::new("00MONDA", 500, err.to_string().as_str()).build()
            })
    }
}