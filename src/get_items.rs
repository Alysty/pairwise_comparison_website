use std::{fs::File, io::{BufRead, BufReader}};

use error_stack::{Result, ResultExt};
use crate::{
    database::database::CONN,
    general_error::errors::GeneralReportError, 
};


pub fn get_items() -> Result<Vec<String>, GeneralReportError> {
    let mut result:Vec<String> = Vec::new();
    let file = File::open("./pairwise_pairing.txt").map_err(GeneralReportError::from_err)?;
    let reader = BufReader::new(file);
    for l in reader.lines() {
        result.push(l.map_err(GeneralReportError::from_err)?);
    }
    Ok(result)
}
// pub fn add_items_to_db() -> Result<Vec<String>, GeneralReportError> {
//     let conn = CONN
//         .lock()
//         .map_err(GeneralReportError::from_err)
//         .attach_printable("Mutex lock failed")?;
//
//     Ok(vec![])
// }
