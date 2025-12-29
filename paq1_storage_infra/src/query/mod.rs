use mongodb::bson::{doc, Document};
use paq1_storage_core::prelude::*;

pub struct DocumentQuery {
    pub filter: Document,
    #[allow(dead_code)]
    pub sorter: Document,
}

impl DocumentQuery {

    #[allow(dead_code)]
    pub fn get_filter(&self) -> &Document {
        &self.filter
    }
}

impl From<Query> for DocumentQuery {
    fn from(value: Query) -> Self {
        let filter = query_to_filter(&value);
        let sorter = query_to_sorter(&value);

        DocumentQuery {
            filter,
            sorter
        }
    }
}

fn query_to_filter(query: &Query) -> Document {
    match query.get_filter() {
        Filter::Expression(e) => {
            let operator = e.get_operator();
            let operator_mongo = from_operation_to_mongo_operator(operator);
            match e {
                Expression::ExpressionString(x) => {
                    doc! { x.field_name.as_str() : { operator_mongo: x.head.as_str() } }
                }
                Expression::ExpressionNumberInt(x) => {
                    doc! { x.field_name.as_str() : { operator_mongo: x.head } }
                }
            }
        },
        Filter::None => doc! {},
    }
}

fn from_operation_to_mongo_operator(operation: &Operation) -> &str {
    let default = "$eq";

    vec![
        (Operation::EqualsTo, "$eq"),
        (Operation::GreaterThan, "$gt"),
        (Operation::LessThan, "$lt"),
    ]
        .into_iter()
        .find(|(op, _)| op == operation)
        .map(|(_, mongo_operator)| mongo_operator)
        .unwrap_or(default)
}

fn query_to_sorter(_query: &Query) -> Document {
    doc! {} // TODO : impl quand on aura dev les sorters :)
}
