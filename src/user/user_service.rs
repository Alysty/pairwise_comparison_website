
use error_stack::{Result, ResultExt};
use rusqlite::params;
use crate::{
    database::database::CONN,
    general_error::errors::GeneralReportError, 
    user::user_structs::User
};


pub fn get_all_users() -> Result<Vec<User>, GeneralReportError> {
    let mut result: Vec<User> = Vec::new();
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("SELECT id, email FROM user")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    let user_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
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

pub fn add_user(email: &String) -> Result<(), GeneralReportError> {
    let conn = CONN
        .lock()
        .map_err(GeneralReportError::from_err)
        .attach_printable("Mutex lock failed")?;
    let mut stmt = conn
        .prepare("INSERT INTO user (email) VALUES (?1)")
        .map_err(GeneralReportError::from_err)
        .attach_printable("Prepared statement failed")?;
    let _ = stmt
        .insert(params![(*email)])
        .map_err(GeneralReportError::from_err)
        .attach_printable("Select failed")?;
    Ok(())
}
