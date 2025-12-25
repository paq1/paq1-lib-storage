use async_trait::async_trait;
use core_lib::daos::DAO;
use core_lib::prelude::Query;

#[derive(Clone, Debug)]
pub struct MockData {
    pub name: String,
    pub age: i32,
}


#[derive(Clone, Debug)]
pub struct MockDataDbo {
    pub name: String,
    pub age: i32,
}

impl From<MockData> for MockDataDbo {
    fn from(value: MockData) -> Self {
        MockDataDbo {
            name: value.name,
            age: value.age,
        }
    }
}

impl From<MockDataDbo> for MockData {
    fn from(value: MockDataDbo) -> Self {
        MockData {
            name: value.name,
            age: value.age,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DaoBuilder {
    pub fetch_one: Result<Option<MockDataDbo>, String>,
    pub fetch_all: Result<Vec<MockDataDbo>, String>,
    pub insert_one: Result<String, String>,
    pub update_one: Result<String, String>,
    pub delete_one: Result<(), String>,
    pub delete_all: Result<(), String>,
    pub count: Result<u64, String>,
}

impl Default for DaoBuilder {
    fn default() -> Self {
        Self {
            fetch_one: Ok(None),
            fetch_all: Ok(vec![]),
            insert_one: Ok("whatever".to_string()),
            update_one: Ok("whatever".to_string()),
            delete_one: Ok(()),
            delete_all: Ok(()),
            count: Ok(1)
        }
    }
}

struct MockDao {
    pub dao_builder: DaoBuilder
}

#[async_trait]
impl DAO<MockDataDbo, String, String> for MockDao {
    async fn fetch_one(&self, _id: &String) -> Result<Option<MockDataDbo>, String> {
        self.dao_builder.fetch_one.clone()
    }

    async fn fetch_all(&self, _query: &Query) -> Result<Vec<MockDataDbo>, String> {
        self.dao_builder.fetch_all.clone()
    }

    async fn insert(&self, _entity: &MockDataDbo) -> Result<String, String> {
        self.dao_builder.insert_one.clone()
    }

    async fn update(&self, _entity: &MockDataDbo) -> Result<String, String> {
        self.dao_builder.update_one.clone()
    }

    async fn delete(&self, _id: &String) -> Result<(), String> {
        self.dao_builder.delete_one.clone()
    }

    async fn delete_all(&self) -> Result<(), String> {
        self.dao_builder.delete_all.clone()
    }

    async fn count(&self) -> Result<u64, String> {
        self.dao_builder.count.clone()
    }
}

impl DaoBuilder {
    pub(super) fn build(&self) -> impl DAO<MockDataDbo, String, String> {
        MockDao {
            dao_builder: self.clone()
        }
    }
}
