use std::collections::HashMap;
use super::global_message_table::GlobalMessage;
use super::event_table::EventTable;
use super::event_type_table::EventTypeTable;
use std::rc::Rc;

pub type TypeTableType = HashMap<u16, &'static str>;

pub trait TypeTableEntry {
    fn get(&self, key: u16) -> &'static str;
}

pub struct TypeTable {
    table: HashMap<&'static str, Rc<TypeTableEntry>>
}

impl TypeTable {
    pub fn new() -> TypeTable {
        TypeTable {
            table: TypeTable::make_table()
        }
    }

    pub fn get(&self, key: &'static str) -> Rc<TypeTableEntry> {
        let result = self.table.get(&key);
        match result {
            Some(r) => r.clone(),
            None => panic!("Unable to lookup type key: {}", key)
        }
    }

    fn make_table() -> HashMap<&'static str, Rc<TypeTableEntry>> {
        let mut table: HashMap<&'static str, Rc<TypeTableEntry>> = HashMap::new();

        let global_message_table = Rc::new(GlobalMessage::new());
        table.insert("global", global_message_table);
        
        let event_table = Rc::new(EventTable::new());
        table.insert("event", event_table);

        let event_type_table = Rc::new(EventTypeTable::new());
        table.insert("event_type", event_type_table);

        table
    }
}


