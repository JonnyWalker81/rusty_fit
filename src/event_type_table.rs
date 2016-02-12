use std::collections::HashMap;
use super::type_table::{TypeTableEntry, TypeTableType};

pub struct EventTypeTable {
    table: TypeTableType
}

impl TypeTableEntry for EventTypeTable {
    fn get(&self, key: u16) -> String {
        let result = self.table.get(&key);
        match result {
            Some(r) => r.clone(),
            None => panic!("Unrecognized event_type: {}", key)
        }
    }
}

impl EventTypeTable {
    pub fn new() -> EventTypeTable {
        EventTypeTable {
            table: EventTypeTable::make_table()
        }
    }

    fn make_table() -> TypeTableType {
        let mut table  = TypeTableType::new();
        table.insert(0, String::from("start"));
        table.insert(1, String::from("stop"));
        table.insert(2, String::from("consecutive_depreciated"));
        table.insert(3, String::from("marker"));
        table.insert(4, String::from("stop_all"));
        table.insert(5, String::from("begin_depreciated"));
        table.insert(6, String::from("end_depreciated"));
        table.insert(7, String::from("end_all_depreciated"));
        table.insert(8, String::from("stop_disable"));
        table.insert(9, String::from("stop_disable_all"));

        table
    }
}
