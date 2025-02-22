use core_lib::repositories::crud_repo::CrudRepository;

pub type MongoRepository<DBO> = CrudRepository<DBO, mongodb::error::Error>;
