use csv::StringRecord;

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub header: StringRecord,
    pub record: Vec<StringRecord>
}

impl Table {
    pub fn new() -> Self {
        Self {
            header: StringRecord::new(),
            record: Vec::new(),
        }
    }
}