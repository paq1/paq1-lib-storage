use crate::integration_tests::mongo::dbo_model::PersonnageDBO;
use crate::integration_tests::mongo::setup::{before_each, set_up_test_personnage_dao};
use paq1_storage_core::prelude::{ExpressionT, Filter};
use paq1_storage_core::query::{Expression, Operation, Pager, Query};

#[tokio::test]
pub async fn should_insert_read_update_delete_in_mongo_test() {

    let collection_name = "test_collection_read_update_delete";

    let dao = set_up_test_personnage_dao(collection_name).await.unwrap();

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

#[tokio::test]
pub async fn should_fetch_all_in_mongo_test() {

    let collection_name = "test_collection_fetch_all";

    let dao = set_up_test_personnage_dao(collection_name).await.unwrap();

    before_each(dao.clone()).await;

    let personnage1 = PersonnageDBO {
        id: "1".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let personnage2 = PersonnageDBO {
        id: "2".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let _ = dao.insert(&personnage1).await.expect("insert failed ... ");
    let _ = dao.insert(&personnage2).await.expect("insert failed ... ");

    let personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::None})
        .await.unwrap();

    assert_eq!(personnages.len(), 2);
    assert_eq!(personnages, vec![personnage1, personnage2]);

    let _ = dao.delete_all().await.unwrap();

    let no_personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::None})
        .await.unwrap();

    assert_eq!(no_personnages.len(), 0);
}

#[tokio::test]
pub async fn should_fetch_all_equals_to_query_in_mongo_test() {

    let collection_name = "test_collection_equals_to_query";

    let dao = set_up_test_personnage_dao(collection_name).await.unwrap();

    before_each(dao.clone()).await;

    let personnage1 = PersonnageDBO {
        id: "1".to_string(),
        name: "whatever".to_string(),
        age: 32,
    };

    let personnage2 = PersonnageDBO {
        id: "2".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let _ = dao.insert(&personnage1).await.expect("insert failed ... ");
    let _ = dao.insert(&personnage2).await.expect("insert failed ... ");

    let personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::Expression(Expression::ExpressionNumberInt(ExpressionT {
            field_name: "age".to_string(),
            operation: Operation::EqualsTo,
            head: 32
        }))})
        .await.unwrap();

    assert_eq!(personnages.len(), 1);
    assert_eq!(personnages, vec![personnage1]);

    let _ = dao.delete_all().await.unwrap();

    let no_personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::None})
        .await.unwrap();

    assert_eq!(no_personnages.len(), 0);
}

#[tokio::test]
pub async fn should_fetch_all_gt_to_query_in_mongo_test() {

    let collection_name = "test_collection_equals_greater_than";

    let dao = set_up_test_personnage_dao(collection_name).await.unwrap();

    before_each(dao.clone()).await;

    let personnage1 = PersonnageDBO {
        id: "1".to_string(),
        name: "whatever".to_string(),
        age: 32,
    };

    let personnage2 = PersonnageDBO {
        id: "2".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let _ = dao.insert(&personnage1).await.expect("insert failed ... ");
    let _ = dao.insert(&personnage2).await.expect("insert failed ... ");

    let personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::Expression(Expression::ExpressionNumberInt(ExpressionT {
            field_name: "age".to_string(),
            operation: Operation::GreaterThan,
            head: 18
        }))})
        .await.unwrap();

    assert_eq!(personnages.len(), 1);
    assert_eq!(personnages, vec![personnage1]);

    let _ = dao.delete_all().await.unwrap();

    let no_personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::None})
        .await.unwrap();

    assert_eq!(no_personnages.len(), 0);
}

#[tokio::test]
pub async fn should_fetch_all_lt_to_query_in_mongo_test() {

    let collection_name = "test_collection_equals_less_than";

    let dao = set_up_test_personnage_dao(collection_name).await.unwrap();

    before_each(dao.clone()).await;

    let personnage1 = PersonnageDBO {
        id: "1".to_string(),
        name: "whatever".to_string(),
        age: 32,
    };

    let personnage2 = PersonnageDBO {
        id: "2".to_string(),
        name: "whatever".to_string(),
        age: 10,
    };

    let _ = dao.insert(&personnage1).await.expect("insert failed ... ");
    let _ = dao.insert(&personnage2).await.expect("insert failed ... ");

    let personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::Expression(Expression::ExpressionNumberInt(ExpressionT {
            field_name: "age".to_string(),
            operation: Operation::LessThan,
            head: 18
        }))})
        .await.unwrap();

    assert_eq!(personnages.len(), 1);
    assert_eq!(personnages, vec![personnage2]);

    let _ = dao.delete_all().await.unwrap();

    let no_personnages: Vec<PersonnageDBO> = dao
        .fetch_all(&Query {pager: Pager::default(), filter: Filter::None})
        .await.unwrap();

    assert_eq!(no_personnages.len(), 0);
}