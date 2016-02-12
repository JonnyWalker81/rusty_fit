use std::collections::HashMap;
use super::field_table::{FieldTableEntry, FieldTableType};

pub struct FileCreatorTable {
    table: FieldTableType
}

impl FieldTableEntry for FileCreatorTable {
    fn get(&self, key: i32) -> String {
        let result = self.table.get(&key);

        match result {
            Some(r) => r.clone(),
            None => panic!("Unrecognized File Creator Key: {}", key)
        }
    }
}

impl FileCreatorTable {
    pub fn new() -> FileCreatorTable {
        FileCreatorTable {
            table: FileCreatorTable::make_table()
        }
    }

    fn make_table() -> FieldTableType {
        let mut file_creator_table = FieldTableType::new();
        file_creator_table.insert(0, String::from("software_version"));
        file_creator_table.insert(1, String::from("hardware_version"));
        file_creator_table
    }
}
