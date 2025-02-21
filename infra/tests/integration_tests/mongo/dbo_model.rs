use serde::{Deserialize, Serialize};
use infra::daos::mongo::identifier::HasIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PersonnageDBO {
    pub id: String,
    pub name: String,
    pub age: i32,
}

impl HasIdentifier for PersonnageDBO {
    fn identifier(&self) -> &String {
        &self.id
    }
}