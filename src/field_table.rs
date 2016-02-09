use std::collections::HashMap;

pub struct FieldTable {
    table: HashMap<&'static str, HashMap<i32, &'static str>>
}

impl FieldTable {
    pub fn new() -> FieldTable {
        FieldTable {
            table: FieldTable::make_table()
        }
    }

    pub fn get(&self, key: &'static str) -> &HashMap<i32, &'static str> {
        let result = self.table.get(&key);
        match result {
            Some(r) => r,
            None => panic!("Unable to get field in table: {}", key)
        }
    }

    fn make_table() -> HashMap<&'static str, HashMap<i32, &'static str>> {
        let mut table = HashMap::new();

        let mut file_id_table: HashMap<i32, &'static str> = HashMap::new();
        file_id_table.insert(0, "type");
        file_id_table.insert(1, "manufacturer");
        file_id_table.insert(2, "product");
        file_id_table.insert(3, "serial_number");
        file_id_table.insert(4, "time_created");
        file_id_table.insert(5, "number");
        file_id_table.insert(8, "product_name");

        table.insert("file_id", file_id_table);

        return table;
    }
}
