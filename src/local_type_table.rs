use std::collections::HashMap;
use super::definition_message::DefinitionMessage;

pub struct LocalMessageTable {
    table: HashMap<u8, DefinitionMessage>
}

impl LocalMessageTable {
    pub fn new() -> LocalMessageTable {
        LocalMessageTable {
            table: HashMap::new()
        }
    }

    pub fn get(&self, key: u8) -> DefinitionMessage {
        let true_key = key as i32;
        let result = self.table.get(&key);
        match result {
            Some(r) => return r.clone(),
            None => panic!("Unable to find: {}", key)
        };
    }

    pub fn insert(&mut self, key:u8, def: DefinitionMessage){
        self.table.insert(key, def);
    }
}
