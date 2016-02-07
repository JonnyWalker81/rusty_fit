use super::record_content::RecordData;
use super::record_header::RecordHeader;

pub struct DataMessage {
    header: RecordHeader,
    data: Vec<u32>
}

impl RecordData for DataMessage {
    fn get_header(&self) -> RecordHeader {
        return self.header.clone();
    }
}
