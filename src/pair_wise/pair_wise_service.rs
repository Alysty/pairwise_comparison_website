use error_stack::{Result, ResultExt};
use rusqlite::params;
use crate::{
    database::database::CONN,
    general_error::errors::GeneralReportError, 
};

use super::pair_wise_structs::{Item, Vote};


pub fn add_vote(vote_info: Vote) -> Result<(), GeneralReportError> {
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("INSERT INTO vote (user_id, win_item_id, lose_item_id) VALUES (?1, ?2, ?3)")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    let _ = stmt
        .insert(params![(vote_info.user_id), (vote_info.win_item_id), (vote_info.lose_item_id)])
        .map_err(GeneralReportError::from_err)
        .attach_printable("Select failed")?;
    Ok(())
}


pub fn get_all_items() -> Result<Vec<Item>, GeneralReportError> {
    let mut result: Vec<Item> = Vec::new();
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("SELECT id, item_name FROM item")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    let user_iter = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                item_name: row.get(1)?,
            })
        })
        .map_err(GeneralReportError::from_err)
        .attach_printable("Select failed")?;
    for user in user_iter {
        result.push(
            user.map_err(GeneralReportError::from_err)
                .attach_printable("Problem with user from select")?,
        );
    }
    Ok(result)
}
