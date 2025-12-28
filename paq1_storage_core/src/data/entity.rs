#[derive(Clone)]
pub struct Entity<DATA> {
    pub data: DATA,
    pub id: String,
    pub version: u32,
}
