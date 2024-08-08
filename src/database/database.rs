use std::sync::Mutex;

use error_stack::{Report, Result, ResultExt};
use lazy_static::lazy_static;
use rusqlite::Connection;

use crate::{general_error::errors::GeneralReportError, user::user_structs::User};

//should probably change it to a pool of connection using r2d2
lazy_static! {
    static ref CONN: Mutex<Connection> = {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE user(
                id INTEGER PRIMARY KEY,
                email TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE items(
                id INTEGER PRIMARY KEY,
                item_name TEXT NOT NULL,
                positive_votes INTEGER NOT NULL
            )",
            (),
        )
        .unwrap();
        Mutex::new(conn)
    };
}

fn get_all_users() -> Result<Vec<User>, GeneralReportError> {
    let mut result: Vec<User> = Vec::new();
    let conn = CONN
        .lock()
        .map_err(|_x| Report::new(GeneralReportError).attach_printable("Mutex lock failed"))?;
    let mut stmt = conn
        .prepare("SELECT id, email FROM user")
        .change_context(GeneralReportError)
        .attach_printable("Prepared statement failed")?;
    let user_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
            })
        })
        .change_context(GeneralReportError)
        .attach_printable("Select failed")?;
    for user in user_iter {
        result.push(
            user.change_context(GeneralReportError)
                .attach_printable("Problem with user from select")?,
        );
    }
    Ok(result)
}
