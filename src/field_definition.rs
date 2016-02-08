
#[derive(Debug)]
pub struct FieldDefinition {
    field_number: u8,
    size: u8,
    base_type_number: u8
}

impl FieldDefinition {
    pub fn new(field_num: u8, s: u8, btn: u8) -> FieldDefinition {
        FieldDefinition {
            field_number: field_num,
            size: s,
            base_type_number: btn
        }
    }
}
