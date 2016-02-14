use std::collections::HashMap;
use std::rc::Rc;
use super::file_id_table::FileIdTable;
use super::file_creator_table::FileCreatorTable;
use super::event_message_table::EventMessageTable;
use super::event_type_table::EventTypeTable;
use super::device_info_table::DeviceInfoTable;
use super::field_table_placeholder::FieldTablePlaceholder;

pub type FieldTableType = HashMap<i32, String>;

pub trait FieldTableEntry {
    fn get(&self, key: i32) -> String;
}

pub struct FieldTable {
    // table: HashMap<&'static str, HashMap<i32, &'static str>>
    table: HashMap<String, Rc<FieldTableEntry>>
}

impl FieldTable {
    pub fn new() -> FieldTable {
        FieldTable {
            table: FieldTable::make_table()
        }
    }

    // pub fn get(&self, key: &'static str) -> &HashMap<i32, &'static str> {
    pub fn get(&self, key: String) -> Rc<FieldTableEntry> {
        let result = self.table.get(&key);
        match result {
            Some(r) => r.clone(),
            None => Rc::new(FieldTablePlaceholder::new())
        }
    }

    // fn make_table() -> HashMap<&'static str, HashMap<i32, &'static str>> {
    fn make_table() -> HashMap<String, Rc<FieldTableEntry>> {
        let mut table: HashMap<String, Rc<FieldTableEntry>> = HashMap::new();

        // let mut file_id_table: HashMap<i32, &'static str> = HashMap::new();
        // file_id_table.insert(0, "type");
        // file_id_table.insert(1, "manufacturer");
        // file_id_table.insert(2, "product");
        // file_id_table.insert(3, "serial_number");
        // file_id_table.insert(4, "time_created");
        // file_id_table.insert(5, "number");
        // file_id_table.insert(8, "product_name");
        // table.insert("file_id", file_id_table);

        let file_table = Rc::new(FileIdTable::new());
        table.insert(String::from("file_id"), file_table);

        // let mut file_creator_table: HashMap<i32, &'static str> = HashMap::new();
        // file_creator_table.insert(0, "software_version");
        // file_creator_table.insert(1, "hardware_version");
        // table.insert("file_creator", file_creator_table);
        let file_creator_table: Rc<FieldTableEntry> = Rc::new(FileCreatorTable::new());
        table.insert(String::from("file_creator"), file_creator_table);

        // let mut event_table: HashMap<i32, &'static str> = HashMap::new();
        // event_table.insert(253, "timestamp");
        // event_table.insert(0, "event");
        // event_table.insert(1, "event_type");
        // event_table.insert(2, "data16");
        // event_table.insert(3, "data");
        // event_table.insert(4, "event_group");
        // event_table.insert(7, "score");
        // event_table.insert(8, "opponent_score");
        // event_table.insert(9, "front_gear_num");
        // event_table.insert(10, "front_gear");
        // event_table.insert(11, "rear_gear_num");
        // event_table.insert(12, "rear_gear");
        // event_table.insert(13, "device_index");
        // table.insert("event", event_table);
        let event_table = Rc::new(EventMessageTable::new());
        table.insert(String::from("event"), event_table);

        let device_info_table = Rc::new(DeviceInfoTable::new());
        table.insert(String::from("device_info"), device_info_table);

        table
    }
}
