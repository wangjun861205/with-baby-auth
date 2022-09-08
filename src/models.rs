#[derive(Debug, Clone)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub salt: String,
}
