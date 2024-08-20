use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub user_id: i64,
    pub win_item_id: i64,
    pub lose_item_id: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub item_name: String
}
