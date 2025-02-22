use mongodb::{Client, Database};

pub struct DatabaseMongo {
    pub underlying: Database,
}

impl DatabaseMongo {
    pub async fn new(uri: &str, dbname: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;

        Ok(Self {
            underlying: client.database(dbname),
        })
    }
}
