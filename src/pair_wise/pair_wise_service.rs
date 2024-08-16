use error_stack::{Result, ResultExt};
use rusqlite::params;
use crate::{
    database::database::CONN,
    general_error::errors::GeneralReportError, 
};

use super::pair_wise_structs::Vote;


pub fn add_vote(vote_info: Vote) -> Result<(), GeneralReportError> {
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("INSERT INTO vote (user_id, item_id) VALUES (?1, ?2)")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    let _ = stmt
        .insert(params![(vote_info.user_id), (vote_info.voted_item_id)])
        .map_err(GeneralReportError::from_err)
        .attach_printable("Select failed")?;
    Ok(())
}
