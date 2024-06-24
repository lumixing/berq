use serde::{Deserialize, Serialize};

use crate::{client::Client, message::Message};

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: String,
}

impl Channel {
    pub async fn create_message(&self, client: &Client, content: &str) {
        client.post(format!("/channels/{}/messages", self.id))
            .send_body(format!("{{\"content\":\"{content}\"}}"))
            .await.unwrap();
    }

    pub async fn create_reply(&self, client: &Client, content: &str, message: Message) {
        client.post(format!("/channels/{}/messages", self.id))
            .send_body(format!("{{\"content\":\"{content}\", \"message_reference\":{{\"message_id\":\"{}\",\"fail_if_not_exists\":false}}}}", message.id))
            .await.unwrap();
    }
}
