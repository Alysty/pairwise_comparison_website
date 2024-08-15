use std::sync::Mutex;

use lazy_static::lazy_static;
use rusqlite::Connection;


//should probably change it to a pool of connection using r2d2
lazy_static! {
    pub static ref CONN: Mutex<Connection> = {
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
