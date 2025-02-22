mod dbo_model;
mod setup;

use crate::integration_tests::mongo::dbo_model::PersonnageDBO;
use crate::integration_tests::mongo::setup::{before_each, set_up_test_personnage_dao};
use core_lib::daos::DAO;
use std::sync::Arc;

#[tokio::test]
pub async fn should_insert_read_update_delete_in_mongo_test() {
    let dao: Arc<dyn DAO<PersonnageDBO, String>> = set_up_test_personnage_dao().await.unwrap();

    before_each(dao.clone()).await;

    let personnage = PersonnageDBO {
        id: "1".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let id = dao.insert(&personnage).await.expect("insert failed ... ");

    assert_eq!(id.to_string(), "1");

    let personnage_dbo = dao
        .fetch_one(&id)
        .await
        .expect("impossible de recuperer la donnée ... ")
        .unwrap();

    assert_eq!(personnage_dbo.name, "whatever");
    assert_eq!(personnage_dbo.age, 10);

    let updated_personnage: PersonnageDBO = PersonnageDBO {
        name: "new name test".to_string(),
        ..personnage_dbo
    };

    dao.update(&updated_personnage).await.expect("update failed ... ");

    let updated_personnage_dbo = dao
        .fetch_one(&id)
        .await
        .expect("impossible de recuperer la donnée ... ")
        .unwrap();

    assert_eq!(updated_personnage_dbo.name, "new name test");
    assert_eq!(updated_personnage_dbo.age, 10);

    dao.delete(&"1".to_string())
        .await
        .expect("delete failed ... ");

    let maybe_personnage_dbo = dao
        .fetch_one(&id)
        .await
        .expect("impossible de recuperer la donnée ... ");
    assert!(maybe_personnage_dbo.is_none());
}
