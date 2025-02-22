#[derive(Clone, Debug)]
pub struct Query {
    pub filter: Filter,
    // TODO : pub sorter: Sorter
    pub pager: Pager,
}

impl Query {
    pub fn get_filter(&self) -> &Filter {
        &self.filter
    }

    pub fn get_pager(&self) -> &Pager {
        &self.pager
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Filter {
    Expression(Expression),
    None
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    ExpressionString(ExpressionT<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpressionT<T> {
    pub field_name: String,
    pub operation: Operation,
    pub head: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    EqualsTo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pager {
    pub page_number: usize,
    pub page_size: usize,
}
impl Default for Pager {
    fn default() -> Self {
        Pager {
            page_size: 10,
            page_number: 0,
        }
    }
}