use std::collections::HashMap;
use super::type_table::{TypeTableEntry, TypeTableType};

pub struct EventTable {
    table: TypeTableType
}

impl TypeTableEntry for EventTable {
    fn get(&self, key: u16) -> String {
        let result = self.table.get(&key);
        match result {
            Some(r) => r.clone(),
            None => panic!("Unrecognized event: {}", key)
        }
    }
}

impl EventTable {
    pub fn new() -> EventTable {
        EventTable {
            table: EventTable::make_table()
        }
    }

    fn make_table() -> TypeTableType {
        let mut table = TypeTableType::new();

        table.insert(0, String::from("timer"));
        table.insert(3, String::from("workout"));
        table.insert(4, String::from("wokrout_step"));
        table.insert(5, String::from("power_down"));
        table.insert(6, String::from("power_up"));
        table.insert(7, String::from("off_course"));
        table.insert(8, String::from("session"));
        table.insert(9, String::from("lap"));
        table.insert(10, String::from("course_point"));
        table.insert(11, String::from("battery"));
        table.insert(12, String::from("virtual_partner_pace"));
        table.insert(13, String::from("hr_high_alert"));
        table.insert(14, String::from("hr_low_alert"));
        table.insert(15, String::from("speed_high_alert"));
        table.insert(16, String::from("speed_low_alert"));
        table.insert(17, String::from("cad_high_alert"));
        table.insert(18, String::from("cad_low_alert"));
        table.insert(19, String::from("power_high_alert"));
        table.insert(20, String::from("power_low_alert"));
        table.insert(21, String::from("recovery_hr"));
        table.insert(22, String::from("battery_low"));
        table.insert(23, String::from("time_duration_alert"));
        table.insert(24, String::from("distance_duration_alert"));
        table.insert(25, String::from("calorie_duration_alert"));
        table.insert(26, String::from("activity"));
        table.insert(27, String::from("fitness_equipment"));
        table.insert(28, String::from("length"));
        table.insert(32, String::from("user_marker"));
        table.insert(33, String::from("sport_point"));
        table.insert(36, String::from("calibration"));
        table.insert(42, String::from("front_gear_change"));
        table.insert(43, String::from("rear_gear_change"));
        table.insert(44, String::from("rider_position_change"));
        table.insert(45, String::from("elev_high_alert"));
        table.insert(46, String::from("elev_low_alert"));
        table.insert(47, String::from("comm_timeout"));

        table
    }
}

