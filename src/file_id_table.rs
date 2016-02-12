use std::collections::HashMap;
use super::field_table::{FieldTableEntry, FieldTableType};

pub struct FileIdTable {
    table: FieldTableType
}

impl FieldTableEntry for FileIdTable {
    fn get(&self, key: i32) -> String {
        let result = self.table.get(&key);

        match result {
            Some(r) => r.clone(),
            None => panic!("Unrecognized File ID Key: {}", key)
        }
    }
}

impl FileIdTable {
    pub fn new() -> FileIdTable {
        return FileIdTable {
            table: FileIdTable::make_table()
        }
    }

    fn make_table() -> FieldTableType {
        let mut table = FieldTableType::new();
        table.insert(0, String::from("type"));
        table.insert(1, String::from("manufacturer"));
        table.insert(2, String::from("product"));
        table.insert(3, String::from("serial_number"));
        table.insert(4, String::from("time_created"));
        table.insert(5, String::from("number"));
        table.insert(7, String::from("product_name"));

        return table;
    }
}
