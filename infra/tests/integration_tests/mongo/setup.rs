use crate::integration_tests::mongo::dbo_model::PersonnageDBO;
use crate::integration_tests::settings::Settings;
use core_lib::daos::DAO;
use infra::daos::mongo::database::DatabaseMongo;
use infra::daos::mongo::mongo_dao::MongoDao;
use paq1_lib_error_handler::prelude::{ErrorWithCodeBuilder, Problem, ResultErr};
use std::sync::Arc;

pub(super) async fn before_each<DBO>(dao: Arc<dyn DAO<DBO, String, mongodb::error::Error>>) {
    dao.delete_all().await.unwrap();
}

pub(super) async fn set_up_test_personnage_dao(
    collection_name: &str,
) -> ResultErr<Arc<dyn DAO<PersonnageDBO, String, mongodb::error::Error>>> {
    let settings = Settings::unsafe_get_lazy();

    let mongo_database = DatabaseMongo::new(settings.database.url.as_str(), "lib_storage_it")
        .await
        .map_err(|e| {
            ErrorWithCodeBuilder::new("00MONCO", 500, "erreur lors de la connection")
                .with_problems(vec![Problem {
                    title: e.to_string(),
                    description: None,
                    warn_message: None,
                }])
                .build()
        })?;

    let collection = mongo_database.underlying.collection(collection_name);
    Ok(Arc::new(MongoDao { collection }))
}

/*
pub(super) async fn set_up_test_personnage_repository(
    collection_name: &str,
) -> ResultErr<Arc<dyn Repository<Personnage, String>>> {
    let dao = set_up_test_personnage_dao(collection_name).await?;

    let repo = Arc::new(MongoRepository { dao: dao.clone() });

    Ok(repo)
}
*/
