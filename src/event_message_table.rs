use std::collections::HashMap;
use super::field_table::{FieldTableEntry, FieldTableType};

pub struct EventMessageTable {
    table: FieldTableType
}

impl FieldTableEntry for EventMessageTable {
    fn get(&self, key: i32) -> String {
        let result = self.table.get(&key);

        match result {
            Some(r) => r.clone(),
            None => panic!("Unrecognized Event Message Key: {}", key)
        }
    }
}

impl EventMessageTable {
    pub fn new() -> EventMessageTable {
        EventMessageTable {
            table: EventMessageTable::make_table()
        }
    }

    fn make_table() -> FieldTableType {
        let mut event_table = FieldTableType::new();
        event_table.insert(253, String::from("timestamp"));
        event_table.insert(0, String::from("event"));
        event_table.insert(1, String::from("event_type"));
        event_table.insert(2, String::from("data16"));
        event_table.insert(3, String::from("data"));
        event_table.insert(4, String::from("event_group"));
        event_table.insert(7, String::from("score"));
        event_table.insert(8, String::from("opponent_score"));
        event_table.insert(9, String::from("front_gear_num"));
        event_table.insert(10, String::from("front_gear"));
        event_table.insert(11, String::from("rear_gear_num"));
        event_table.insert(12, String::from("rear_gear"));
        event_table.insert(13, String::from("device_index"));
        event_table
    }
}
