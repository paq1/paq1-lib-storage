use serde::{Deserialize, Serialize};
use paq1_storage_core::data::event::Event;
use crate::daos::mongo::identifier::HasIdentifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDBO<EVT> {
    #[allow(dead_code)]
    pub data: EVT,
    pub id: String,
    pub entity_id: String,
    #[allow(dead_code)]
    pub version: u32,
}

impl<EVT> HasIdentifier for EventDBO<EVT> {
    fn identifier_key() -> String {
        "id".to_string()
    }

    fn identifier_value(&self) -> &String {
        &self.id
    }
}


impl<A, B> From<Event<A>> for EventDBO<B>
where
    A: From<B>,
    B: From<A>
{
    fn from(value: Event<A>) -> Self {
        Self {
            data: value.data.into(),
            id: value.id.clone(),
            entity_id: value.entity_id.clone(),
            version: value.version.clone(),
        }
    }
}

impl<A, B> From<EventDBO<A>> for Event<B>
where
    A: From<B>,
    B: From<A>
{
    fn from(value: EventDBO<A>) -> Self {
        Self {
            data: value.data.into(),
            id: value.id.clone(),
            entity_id: value.entity_id.clone(),
            version: value.version.clone(),
        }
    }
}
