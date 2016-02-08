use super::record_content::RecordData;
use super::record_header::RecordHeader;
use super::record_datum::RecordDatum;

#[derive(Debug)]
pub struct DataMessage {
    header: RecordHeader,
    data: Vec<RecordDatum>
}

impl DataMessage {
    pub fn new(raw_header: RecordHeader) -> DataMessage {
        DataMessage {
            header: raw_header,
            data: Vec::new()
        }
    }

    pub fn push_datum(&mut self, d: RecordDatum) {
        self.data.push(d);
    }
}

impl RecordData for DataMessage {
    fn get_header(&self) -> RecordHeader {
        return self.header.clone();
    }
}
