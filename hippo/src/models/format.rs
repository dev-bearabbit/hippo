use csv::StringRecord;

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub header: StringRecord,
    pub record: Vec<StringRecord>
}
