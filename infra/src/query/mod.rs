use core_lib::prelude::*;
use mongodb::bson::{doc, Document};

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
        Filter::Expression(e) => match e {
            Expression::ExpressionString(x) =>
                doc! { x.field_name.as_str() : x.head.as_str() },
        },
        Filter::None => doc! {},
    }
}

fn query_to_sorter(_query: &Query) -> Document {
    doc! {} // TODO : impl quand on aura dev les sorters :)
}
