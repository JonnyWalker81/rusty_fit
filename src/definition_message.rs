use super::record_content::RecordData;
use super::record_header::RecordHeader;
use super::field_definition::FieldDefinition;

#[derive(Debug)]
pub struct DefinitionMessage {
    header: RecordHeader,
    pub architecture: u8,
    pub global_message_number: u16,
    pub number_of_fields: u8,
    pub fields: Vec<FieldDefinition>
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

