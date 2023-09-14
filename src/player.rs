use serde::Deserialize;
use serde::Serialize;
#[derive(Debug,Deserialize,Serialize)]
pub(crate) struct Player {
    id: i64,
    score: i64,
    username: String,
}
impl Player {
    pub fn new(id: i64, score: i64, username: String) -> Self {
        Self{
            id,
            score,
            username
        }
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn get_score(&self) -> i64 {
        self.score
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn add_to_score(&mut self, amount: i64) {
        self.score += amount;
    }
    pub fn sub_from_score(&mut self, amount: i64) {
        self.score -= amount;
    }
}
