use crate::data::quick_search::QuickSearchPath;
use crate::query::Query;
use async_trait::async_trait;

#[async_trait]
pub trait DAO<DBO, ID, ERR>: Send + Sync {

    async fn fetch_one(&self, id: &ID) -> Result<Option<DBO>, ERR>;
    async fn fetch_all(&self, query: &Query) -> Result<Vec<DBO>, ERR>;

    async fn insert(&self, entity: &DBO) -> Result<ID, ERR>;
    async fn update(&self, entity: &DBO) -> Result<ID, ERR>;

    async fn delete(&self, id: &String) -> Result<(), ERR>;
    async fn delete_all(&self) -> Result<(), ERR>;


    async fn quick_search(&self, chaine: &str, paths: Vec<QuickSearchPath>) -> Result<Vec<DBO>, ERR>;
    async fn count(&self) -> Result<u64, ERR>;

}
