use super::record_content::RecordData;
use super::record_header::RecordHeader;
use super::field_definition::FieldDefinition;

pub struct DefinitionMessage {
    header: RecordHeader,
    architecture: u8,
    global_message_number: u16,
    number_of_fields: u8,
    fields: Vec<FieldDefinition>
}

impl DefinitionMessage {
    pub fn new(h: RecordHeader) -> DefinitionMessage {
        DefinitionMessage {
            header: h,
            architecture: 0,
            global_message_number: 0,
            number_of_fields: 0,
            fields: Vec::new()
        }
    }
}

impl RecordData for DefinitionMessage {
    fn get_header(&self) -> RecordHeader {
        return self.header.clone();
    }
}

