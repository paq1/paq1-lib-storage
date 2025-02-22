use async_trait::async_trait;
use paq1_lib_error_handler::prelude::ResultErr;
use crate::prelude::Query;

#[async_trait]
pub trait Repository<DATA, ID> {

    async fn fetch_one(&self, id: &ID) -> ResultErr<Option<DATA>>;
    async fn fetch_all(&self, query: &Query) -> ResultErr<Vec<DATA>>;

    async fn insert(&self, entity: &DATA) -> ResultErr<ID>;
    async fn update(&self, entity: &DATA) -> ResultErr<ID>;

    async fn delete(&self, id: &String) -> ResultErr<()>;
    async fn delete_all(&self) -> ResultErr<()>;

}
