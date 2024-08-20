use std::{fs::File, io::{BufRead, BufReader}};

use error_stack::{Result, ResultExt};
use rusqlite::params;
use crate::{
    database::database::CONN,
    general_error::errors::GeneralReportError, 
};


pub fn add_items_to_db() -> Result<(), GeneralReportError> {
    let mut items:Vec<String> = Vec::new();
    let file = File::open("./pairwise_pairing.txt").map_err(GeneralReportError::from_err)?;
    let reader = BufReader::new(file);
    for l in reader.lines() {
        items.push(l.map_err(GeneralReportError::from_err)?);
    }
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("INSERT INTO item (item_name) VALUES (?1)")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    for i in items {
        let _ = stmt
        .insert(params![(i)])
        .map_err(GeneralReportError::from_err)
        .attach_printable("insert item failed for item ".to_string() + i.as_str())?;
    }
    Ok(())
}



