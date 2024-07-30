use csv::StringRecord;

#[derive(Debug, Clone)]
pub struct Table {
    pub header: StringRecord,
    pub record: Vec<StringRecord>
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadPressed,
    CsvLoaded(Result<Table, String>),
}
