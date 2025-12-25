#[derive(Debug, Clone)]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub total_page: u32,
    pub total_records: u32,
    pub offset: u32,
    pub page_size: u32,
}
