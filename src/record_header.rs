
#[derive(Clone, Debug)]
pub struct RecordHeader {
    header_type: u8,
    message_type: u8,
    local_message_type: u8
}

impl RecordHeader {
    pub fn new(raw_header: u8) -> RecordHeader {
        let t = (raw_header & 0b10000000) >> 7;
        let mt = (raw_header & 0b01000000) >> 6;
        let lmt = (raw_header & 0b00000111);

        RecordHeader {
            header_type: t,
            message_type: mt,
            local_message_type: lmt
        }
    }

    pub fn is_definition(&self) -> bool {
        self.message_type == 1
    }
}
