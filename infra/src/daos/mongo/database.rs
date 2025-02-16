use mongodb::{Client, Database};
use paq1_lib_error_handler::prelude::*;

pub struct DatabaseMongo {
    pub underlying: Database,
}

impl DatabaseMongo {
    pub async fn new(uri: &str, dbname: &str) -> ResultErr<Self> {
        let client = Client::with_uri_str(uri).await.map_err(|err| {
            ErrorWithCodeBuilder::new("00MGCONCLI", 500, "connection au client mongo en erreur")
                .with_problems(vec![Problem {
                    title: format!("mongo error : {err}"),
                    description: None,
                    warn_message: None,
                }])
                .build()
        })?;

        Ok(Self {
            underlying: client.database(dbname),
        })
    }
}
