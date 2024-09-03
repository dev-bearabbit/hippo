use polars::prelude::*;
use std::path::PathBuf;
use crate::models::error::TableError;
use calamine::{open_workbook_auto, Reader, Data};

#[derive(Debug, Clone, Default)]
pub struct RecordTable {
    pub dataframe: DataFrame
}

impl RecordTable {
    pub fn new() -> Self {
        Self {
            dataframe: DataFrame::empty()
        }
    }

    pub fn from_csv(self, path: PathBuf) -> Result<Self, TableError>  {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path))
            .unwrap()
            .finish()?;

            Ok(RecordTable{dataframe: df})
    }

    pub fn from_excel(self, path: PathBuf) -> Result<Self, TableError>  {
        let file_path = path.as_path();
        let mut workbook = open_workbook_auto(file_path).map_err(|e| TableError::Calamine(e.into()))?;
        
        let range = workbook.worksheet_range_at(0)
            .ok_or_else(|| TableError::NotFound("Cannot find the first sheet".to_string()))?
            .map_err(|e| TableError::Calamine(e.into()))?;

        let headers: Vec<String> = range.rows().next()
            .ok_or_else(|| TableError::NotFound("Sheet is empty".to_string()))?
            .iter()
            .map(|cell| cell.to_string())
            .collect();

        let mut columns: Vec<Vec<Data>> = vec![Vec::new(); headers.len()];

        for row in range.rows().skip(1) { 
            for (i, cell) in row.iter().enumerate() {
                columns[i].push(cell.clone());
            }
        }

       let mut series_vec = Vec::with_capacity(headers.len());

       for (header, column) in headers.iter().zip(columns) {
           let series = match column.get(0) {
               Some(Data::Int(_)) => {
                   Series::new(header, column.iter().map(|x| match x {
                    Data::Int(i) => *i as i64,
                       _ => 0,
                   }).collect::<Vec<_>>())
               },
               Some(Data::Float(_)) => {
                   Series::new(header, column.iter().map(|x| match x {
                    Data::Float(f) => *f,
                       _ => 0.0,
                   }).collect::<Vec<_>>())
               },
               Some(Data::String(_)) => {
                   Series::new(header, column.iter().map(|x| match x {
                    Data::String(s) => s.clone(),
                       _ => "".to_string(),
                   }).collect::<Vec<_>>())
               },
               Some(Data::Bool(_)) => {
                   Series::new(header, column.iter().map(|x| match x {
                    Data::Bool(b) => *b,
                       _ => false,
                   }).collect::<Vec<_>>())
               },
               _ => Series::new(header, column.iter().map(|_| "".to_string()).collect::<Vec<_>>()),
           };
           series_vec.push(series);
       }

       let df = DataFrame::new(series_vec).map_err(TableError::from)?;

       Ok(RecordTable { dataframe: df })
   }
}