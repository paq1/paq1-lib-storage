#[derive(Clone)]
pub struct Event<EVT> {
    pub data: EVT,
    pub id: String,
    pub entity_id: String,
    pub version: u32,
}
