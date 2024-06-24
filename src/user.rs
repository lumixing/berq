use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: Option<bool>, // maybe serde optional?
}

impl User {
    pub fn is_bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }
}
