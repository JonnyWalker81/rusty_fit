use super::field_table::{FieldTableEntry, FieldTableType};

pub struct DeviceInfoTable {
    table: FieldTableType
}

impl FieldTableEntry for DeviceInfoTable {
    fn get(&self, key: i32) -> String {
        let result = self.table.get(&key);

        match result {
            Some(r) => r.clone(),
            // None => panic!("Unrecognized Devce Info Key: {}", key)
            None => {
                // let s = format!("{}", key);
                // s.as_str().clone()
                key.to_string()
            }
        }
    }
}

impl DeviceInfoTable {
    pub fn new() -> DeviceInfoTable {
        DeviceInfoTable {
            table: DeviceInfoTable::make_table()
        }
    }

    fn make_table() -> FieldTableType {
        let mut table = FieldTableType::new();
        table.insert(253, String::from("timestamp"));
        table.insert(0, String::from("device_index"));
        table.insert(1, String::from("device_type"));
        table.insert(2, String::from("manufacturer"));
        table.insert(3, String::from("serial_number"));
        table.insert(4, String::from("product"));
        table.insert(5, String::from("software_version"));
        table.insert(6, String::from("hardware_version"));
        table.insert(7, String::from("cum_operating_time"));
        table.insert(10, String::from("battery_voltage"));
        table.insert(11, String::from("battery_status"));
        table.insert(18, String::from("sensor_position"));
        table.insert(19, String::from("descriptor"));
        table.insert(20, String::from("ant_transmission_type"));
        table.insert(21, String::from("ant_device_number"));
        table.insert(22, String::from("ant_network"));
        table.insert(25, String::from("source_type"));
        table.insert(27, String::from("product_name"));

        table
    }
}
