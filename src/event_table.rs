use std::collections::HashMap;
use super::type_table::TypeTableEntry;

pub struct EventTable {
    table: HashMap<u16, &'static str>
}

impl TypeTableEntry for EventTable {
    fn get(&self, key: u16) -> &'static str {
        let result = self.table.get(&key);
        match result {
            Some(r) => r,
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

    fn make_table() -> HashMap<u16, &'static str> {
        let mut table: HashMap<u16, &'static str> = HashMap::new();

        table.insert(0, "timer");
        table.insert(3, "workout");
        table.insert(4, "wokrout_step");
        table.insert(5, "power_down");
        table.insert(6, "power_up");
        table.insert(7, "off_course");
        table.insert(8, "session");
        table.insert(9, "lap");
        table.insert(10, "course_point");
        table.insert(11, "battery");
        table.insert(12, "virtual_partner_pace");
        table.insert(13, "hr_high_alert");
        table.insert(14, "hr_low_alert");
        table.insert(15, "speed_high_alert");
        table.insert(16, "speed_low_alert");
        table.insert(17, "cad_high_alert");
        table.insert(18, "cad_low_alert");
        table.insert(19, "power_high_alert");
        table.insert(20, "power_low_alert");
        table.insert(21, "recovery_hr");
        table.insert(22, "battery_low");
        table.insert(23, "time_duration_alert");
        table.insert(24, "distance_duration_alert");
        table.insert(25, "calorie_duration_alert");
        table.insert(26, "activity");
        table.insert(27, "fitness_equipment");
        table.insert(28, "length");
        table.insert(32, "user_marker");
        table.insert(33, "sport_point");
        table.insert(36, "calibration");
        table.insert(42, "front_gear_change");
        table.insert(43, "rear_gear_change");
        table.insert(44, "rider_position_change");
        table.insert(45, "elev_high_alert");
        table.insert(46, "elev_low_alert");
        table.insert(47, "comm_timeout");

        table
    }
}

