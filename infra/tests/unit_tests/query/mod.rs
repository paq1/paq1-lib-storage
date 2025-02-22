use core_lib::query::*;
use infra::prelude::DocumentQuery;
use mongodb::bson::doc;

#[test]
pub fn should_map_query_filter_empty_to_empty_document_test() {


    let query: Query = Query {
        filter: Filter::None,
        pager: Pager::default()
    };

    let document_wrapper: DocumentQuery = query.into();

    assert_eq!(document_wrapper.filter, doc! {});
}

#[test]
pub fn should_map_query_filter_equal_to_filter_equal_document_test() {


    let query: Query = Query {
        filter: Filter::Expression(Expression::ExpressionString(
            ExpressionT::<String> {
                field_name: "field_test".to_lowercase(),
                operation: Operation::EqualsTo,
                head: "value test".to_lowercase()
            }
        )),
        pager: Pager::default()
    };

    let document_wrapper: DocumentQuery = query.into();

    assert_eq!(document_wrapper.filter, doc! {
        "field_test": "value test"
    });
}

