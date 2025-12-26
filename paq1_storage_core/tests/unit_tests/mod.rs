use paq1_storage_core::prelude::{Filter, Pager, Query};

pub mod repositories;

#[test]
fn should_get_filer_and_pager() {
    let query = Query {
        filter: Filter::None,
        pager: Pager::default()
    };

    assert_eq!(query.get_filter(), &Filter::None);
    assert_eq!(query.get_pager(), &Pager::default());
}
