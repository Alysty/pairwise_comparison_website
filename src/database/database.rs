use std::sync::Mutex;

use lazy_static::lazy_static;
use rusqlite::Connection;

// TODO: should probably change it to a pool of connection using r2d2
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
            "CREATE TABLE item(
                id INTEGER PRIMARY KEY,
                item_name TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE vote(
                win_item_id INTEGER,
                lose_item_id INTEGER,
                user_id INTEGER,
                PRIMARY KEY (lose_item_id, win_item_id, user_id),
                FOREIGN KEY (win_item_id) references item(id),
                FOREIGN KEY (lose_item_id) references item(id),
                FOREIGN KEY (user_id) references user(id)
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE INDEX idx_win_item_id ON vote (win_item_id)",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE INDEX idx_lose_item_id ON vote (lose_item_id)",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE INDEX idx_vote_user_id ON vote (user_id)",
            (),
        )
        .unwrap();
        Mutex::new(conn)
    };
}
