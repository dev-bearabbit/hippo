use crate::models::format::Table;

use csv::{Reader, StringRecord};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use native_dialog::FileDialog;

pub fn load_csv_from_dialog() -> Result<Table, String>{
    if let Ok(Some(path)) = FileDialog::new()
        .add_filter("CSV file", &["csv"])
        .show_open_single_file()
    {
        _load_csv(path)
    } else {
        Err(String::from("No file selected"))
    }
}

fn _load_csv(path: PathBuf) -> Result<Table, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let buf_reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(buf_reader);

    // 헤더 읽기
    let headers = rdr.headers().map_err(|e| e.to_string())?.clone();

    // 데이터 저장용 벡터
    let mut data: Vec<StringRecord> = Vec::new();

    // 각 레코드를 StringRecord로 변환하여 벡터에 저장
    for result in rdr.records() {
        let row = result.map_err(|e| e.to_string())?;
        data.push(row);
    }

    // // 데이터가 잘 읽혔는지 확인
    // for row in &data {
    //     println!("{:?}", row);
    // }

    // 마지막으로 정리
    let output = Table{ header: headers, record: data};

    Ok(output)
}
