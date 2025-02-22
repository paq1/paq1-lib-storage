use infra::daos::mongo::identifier::HasIdentifier;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Personnage {
    pub id: String,
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

impl From<PersonnageDBO> for Personnage {
    fn from(value: PersonnageDBO) -> Self {
        Personnage {
            id: value.id,
            name: value.name,
            age: value.age,
        }
    }
}

impl From<Personnage> for PersonnageDBO {
    fn from(value: Personnage) -> Self {
        PersonnageDBO {
            id: value.id,
            name: value.name,
            age: value.age,
        }
    }
}
