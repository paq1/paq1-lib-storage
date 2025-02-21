mod dbo_model;

use crate::integration_tests::mongo::dbo_model::PersonnageDBO;
use crate::integration_tests::settings::Settings;
use core_lib::daos::DAO;
use infra::daos::mongo::database::DatabaseMongo;
use infra::daos::mongo::mongo_dao::MongoDao;

#[tokio::test]
pub async fn should_insert_read_update_delete_in_mongo_test() {

    let settings =  Settings::unsafe_get_lazy();

    let mongo_database = DatabaseMongo::new(settings.database.url.as_str(), "lib_storage_it").await.unwrap();
    let collection = mongo_database.underlying.collection("lib_storage_it_insertion");
    let dao: MongoDao<PersonnageDBO> = MongoDao { collection };

    let personnage = PersonnageDBO { id: "1".to_string(), name: "whatever".to_string(), age: 10 };

    let id = dao.insert(&personnage).await.expect("insert failed ... ");

    assert_eq!(id.to_string(), "1");

    let personnage_dbo = dao.fetch_one(&id).await.expect("impossible de recuperer la donnée ... ").unwrap();

    assert_eq!(personnage_dbo.name, "whatever");
    assert_eq!(personnage_dbo.age, 10);

    dao.delete(&"1".to_string()).await.expect("delete failed ... ");

    let maybe_personnage_dbo = dao.fetch_one(&id).await.expect("impossible de recuperer la donnée ... ");
    assert!(maybe_personnage_dbo.is_none());
}
