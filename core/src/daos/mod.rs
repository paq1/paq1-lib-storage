use crate::query::Query;
use async_trait::async_trait;
use paq1_lib_error_handler::prelude::ResultErr;

#[async_trait]
pub trait DAO<DBO, ID>: Send + Sync {

    async fn fetch_one(&self, id: &ID) -> ResultErr<Option<DBO>>;
    async fn fetch_all(&self, query: &Query) -> ResultErr<Vec<DBO>>;

    async fn insert(&self, entity: &DBO) -> ResultErr<ID>;
    async fn update(&self, entity: &DBO) -> ResultErr<ID>;

    async fn delete(&self, id: &String) -> ResultErr<()>;
    async fn delete_all(&self) -> ResultErr<()>;

}
