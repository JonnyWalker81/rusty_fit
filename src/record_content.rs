use super::record_header::RecordHeader;

pub trait RecordData {
    fn get_header(&self) -> RecordHeader;
}
pub struct RecordContent {
    records: Vec<Box<RecordData>>
}

impl RecordContent {
    pub fn new() -> RecordContent {
        return RecordContent {
            records: Vec::new()
        }
    }
}
