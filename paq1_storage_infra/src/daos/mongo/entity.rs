use serde::{Deserialize, Serialize};
use paq1_storage_core::data::entity::Entity;
use crate::daos::mongo::identifier::HasIdentifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDBO<DATA> {
    #[allow(dead_code)]
    pub data: DATA,
    pub id: String,
    #[allow(dead_code)]
    pub version: u32,
}

impl<DATA> HasIdentifier for EntityDBO<DATA> {
    fn identifier_key() -> String {
        "id".to_string()
    }

    fn identifier_value(&self) -> &String {
        &self.id
    }
}


impl<A, B> From<Entity<A>> for EntityDBO<B>
where
    A: From<B>,
    B: From<A>
{
    fn from(value: Entity<A>) -> Self {
        Self {
            data: value.data.into(),
            id: value.id.clone(),
            version: value.version.clone(),
        }
    }
}

impl<A, B> From<EntityDBO<A>> for Entity<B>
where
    A: From<B>,
    B: From<A>
{
    fn from(value: EntityDBO<A>) -> Self {
        Self {
            data: value.data.into(),
            id: value.id.clone(),
            version: value.version.clone(),
        }
    }
}
