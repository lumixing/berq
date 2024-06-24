use crate::{channel::Channel, user::User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub channel_id: String,
    pub author: User,
}

impl Message {
    pub fn get_channel(&self) -> Channel {
        Channel {
            id: self.channel_id.clone(),
        }
    }
}
