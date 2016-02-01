use std::collections::HashMap;

pub struct FileIdMessage {
    table: HashMap<u8, &'static str>
}

impl FileIdMessage {
    pub fn new() -> FileIdMessage {
        return FileIdMessage {
            table: FileIdMessage::make_table()
        }
    }

    pub fn get(&self, key: u8) -> &'static str {
        let result = self.table.get(&key);

        match result {
            Some(r) => return r,
            None => panic!("Unrecognized File ID Key: {}", key)
        };
    }

    fn make_table() -> HashMap<u8, &'static str> {
        let mut table = HashMap::new();
        table.insert(0, "type");
        table.insert(1, "manufacturer");
        table.insert(2, "product");
        table.insert(3, "serial_number");
        table.insert(4, "time_created");
        table.insert(5, "number");
        table.insert(7, "product_name");

        return table;
    }
}
