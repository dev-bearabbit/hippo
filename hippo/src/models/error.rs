use std::fmt;
use std::error::Error;
use polars::prelude::PolarsError;
use calamine::Error as CalamineError;

#[derive(Debug)]
pub enum TableError {
    Io(std::io::Error),
    Polars(PolarsError),
    Calamine(CalamineError),
    NotFound(String),
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableError::Io(err) => write!(f, "IO error: {}", err),
            TableError::Polars(err) => write!(f, "Polars error: {}", err),
            TableError::Calamine(err) => write!(f, "Calamine error: {}", err),
            TableError::NotFound(err) => write!(f, "Not Found Error: {}", err),
        }
    }
}

impl Error for TableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TableError::Io(err) => Some(err),
            TableError::Polars(err) => Some(err),
            TableError::Calamine(err) => Some(err),
            TableError::NotFound(_) => None,
        }
    }
}

// 각 에러 타입에 대한 변환 구현
impl From<std::io::Error> for TableError {
    fn from(error: std::io::Error) -> Self {
        TableError::Io(error)
    }
}

impl From<PolarsError> for TableError {
    fn from(error: PolarsError) -> Self {
        TableError::Polars(error)
    }
}

impl From<CalamineError> for TableError {
    fn from(error: CalamineError) -> Self {
        TableError::Calamine(error)
    }
}
