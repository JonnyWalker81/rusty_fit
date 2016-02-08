
#[derive(Debug)]
pub struct RecordDatum {
    name: String,
    value: i64
}

impl RecordDatum {
    pub fn new(n: String, v: i64) -> RecordDatum {
        RecordDatum {
            name: n,
            value: v
        }
    }
}
