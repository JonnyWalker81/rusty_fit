use super::field_table::{FieldTableType, FieldTableEntry};

pub struct FieldTablePlaceholder {
    table: FieldTableType
}

impl FieldTableEntry for FieldTablePlaceholder {
    fn get(&self, key: i32) -> String {
        key.to_string()
    }
}

impl FieldTablePlaceholder {
    pub fn new() -> FieldTablePlaceholder {
        FieldTablePlaceholder {
            table: FieldTableType::new()
        }
    }
}
