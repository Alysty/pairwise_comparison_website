use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
}
