use paq1_storage_core::repositories::crud_repo::CrudRepository;

pub type MongoRepository<DBO> = CrudRepository<DBO, mongodb::error::Error>;
