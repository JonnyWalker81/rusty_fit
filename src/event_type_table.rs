use std::collections::HashMap;
use super::type_table::{TypeTableEntry, TypeTableType};

pub struct EventTypeTable {
    table: TypeTableType
}

impl TypeTableEntry for EventTypeTable {
    fn get(&self, key: u16) -> &'static str {
        let result = self.table.get(&key);
        match result {
            Some(r) => r,
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
        let mut table: TypeTableType = TypeTableType::new();
        table.insert(0, "start");
        table.insert(1, "stop");
        table.insert(2, "consecutive_depreciated");
        table.insert(3, "marker");
        table.insert(4, "stop_all");
        table.insert(5, "begin_depreciated");
        table.insert(6, "end_depreciated");
        table.insert(7, "end_all_depreciated");
        table.insert(8, "stop_disable");
        table.insert(9, "stop_disable_all");

        table
    }
}
