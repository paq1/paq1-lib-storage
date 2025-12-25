use crate::unit_tests::repositories::mock_dao::{DaoBuilder, MockData, MockDataDbo};
use core_lib::daos::DAO;
use core_lib::prelude::crud_repo::CrudRepository;
use core_lib::prelude::Filter;
use core_lib::query::{Pager, Query};
use core_lib::repositories::repository::Repository;
use paq1_lib_error_handler::prelude::Error;
use std::sync::Arc;

mod mock_dao;

#[tokio::test]
async fn should_fetch_one_none_when_dao_fetch_none_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_one: Ok(None),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_data = repository.fetch_one(&"whatever".to_string()).await.unwrap();
    assert!(maybe_data.is_none());
}

#[tokio::test]
async fn should_fetch_one_item_when_dao_fetch_item_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_one: Ok(Some(MockDataDbo {
                name: "item".to_string(),
                age: 18,
            })),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_data = repository
        .fetch_one(&"whatever".to_string())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(maybe_data.age, 18);
    assert_eq!(maybe_data.name, "item");
}

#[tokio::test]
async fn should_fetch_one_error_when_dao_fetch_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_one: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository.fetch_one(&"whatever".to_string()).await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONFO"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}

#[tokio::test]
async fn should_fetch_all_empty_when_dao_fetch_empty_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_all: Ok(vec![]),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_data = repository
        .fetch_all(&Query {
            filter: Filter::None,
            pager: Pager::default(),
        })
        .await
        .unwrap();
    assert!(maybe_data.data.is_empty());
}

#[tokio::test]
async fn should_fetch_all_items_when_dao_fetch_items_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_all: Ok(vec![MockDataDbo {
                name: "item".to_string(),
                age: 18,
            }]),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let paged = repository
        .fetch_all(&Query {
            filter: Filter::None,
            pager: Pager::default(),
        })
        .await
        .unwrap();

    let data = paged.data;

    assert_eq!(data.len(), 1);

    assert_eq!(data[0].age, 18);
    assert_eq!(data[0].name, "item");
}

#[tokio::test]
async fn should_fetch_all_error_when_dao_fetch_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            fetch_all: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository
        .fetch_all(&Query {
            filter: Filter::None,
            pager: Pager::default(),
        })
        .await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONFA"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}

#[tokio::test]
async fn should_ok_when_dao_insert_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            insert_one: Ok("id".to_string()),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let id_inserted = repository
        .insert(&MockData {
            name: "whatever".to_string(),
            age: 18,
        })
        .await
        .unwrap();

    assert_eq!(id_inserted, String::from("id"));
}

#[tokio::test]
async fn should_failed_insert_when_dao_insert_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            insert_one: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository
        .insert(&MockData {
            name: "whatever".to_string(),
            age: 18,
        })
        .await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONIN"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}

#[tokio::test]
async fn should_ok_when_dao_update_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            update_one: Ok("id".to_string()),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let id_updated = repository
        .update(&MockData {
            name: "whatever".to_string(),
            age: 18,
        })
        .await
        .unwrap();

    assert_eq!(id_updated, String::from("id"));
}

#[tokio::test]
async fn should_failed_update_when_dao_update_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            update_one: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository
        .update(&MockData {
            name: "whatever".to_string(),
            age: 18,
        })
        .await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONUP"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}

#[tokio::test]
async fn should_ok_when_dao_delete_one_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            delete_one: Ok(()),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let acknowledge = repository.delete(&String::from("whatever")).await;

    assert!(acknowledge.is_ok());
}

#[tokio::test]
async fn should_failed_delete_one_when_dao_delete_one_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            delete_one: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository.delete(&String::from("whatever")).await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONDO"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}

#[tokio::test]
async fn should_ok_when_dao_delete_all_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            delete_all: Ok(()),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let acknowledge = repository.delete_all().await;

    assert!(acknowledge.is_ok());
}

#[tokio::test]
async fn should_failed_delete_all_when_dao_delete_all_error_test() {
    let mock_dao: Arc<dyn DAO<MockDataDbo, String, String>> = Arc::new(
        DaoBuilder {
            delete_all: Err(String::from("error message from dao")),
            ..Default::default()
        }
        .build(),
    );

    let repository: Arc<dyn Repository<MockData, String>> = Arc::new(CrudRepository {
        dao: mock_dao.clone(),
    });

    let maybe_ok = repository.delete_all().await;

    assert!(maybe_ok.is_err());

    match maybe_ok {
        Ok(_) => panic!("Should have failed!"),
        Err(Error::ErrorWithCode(e)) => {
            assert_eq!(e.title, String::from("error message from dao"));
            assert_eq!(e.code, String::from("00MONDA"));
            assert_eq!(e.status, 500);
            assert_eq!(e.description, None);
            assert_eq!(e.problems, vec![]);
        }
    }
}
