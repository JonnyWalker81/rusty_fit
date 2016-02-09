
#[derive(Copy, Clone, Debug)]
pub struct FieldDefinition {
    pub field_number: u8,
    pub size: u8,
    pub base_type_number: u8
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
