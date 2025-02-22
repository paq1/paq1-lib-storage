use std::sync::Arc;
use paq1_lib_error_handler::prelude::ResultErr;
use core_lib::daos::DAO;
use infra::daos::mongo::database::DatabaseMongo;
use infra::daos::mongo::mongo_dao::MongoDao;
use crate::integration_tests::mongo::dbo_model::PersonnageDBO;
use crate::integration_tests::settings::Settings;

pub(super) async fn before_each<DBO>(dao: Arc<dyn DAO<DBO, String>>) {
    dao.delete_all().await.unwrap();
}

pub(super) async fn set_up_test_personnage_dao() -> ResultErr<Arc<dyn DAO<PersonnageDBO, String>>> {
    let settings = Settings::unsafe_get_lazy();

    let mongo_database = DatabaseMongo::new(settings.database.url.as_str(), "lib_storage_it")
        .await
        .unwrap();
    let collection = mongo_database
        .underlying
        .collection("lib_storage_it_insertion");
    let dao: Arc<dyn DAO<PersonnageDBO, String>> = Arc::new(MongoDao { collection });
    Ok(dao)
}