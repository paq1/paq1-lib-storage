use infra::daos::mongo::identifier::HasIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PersonnageDBO {
    pub id: String,
    pub name: String,
    pub age: i32,
}

impl HasIdentifier for PersonnageDBO {
    fn identifier_value(&self) -> &String {
        &self.id
    }

    fn identifier_key() -> String {
        String::from("id")
    }
}