use crate::event::{Data, GatewayEvent, Properties};

const API_BASE: &str = "https://discord.com/api";

pub struct Client {
    token: String,
    pub http_client: awc::Client,
    pub ws_client: awc::Client,
}

impl Client {
    pub fn new(token: String) -> Self {
        let http_client = awc::Client::default();
        let ws_client = awc::Client::builder()
            .max_http_version(awc::http::Version::HTTP_11)
            .finish();

        Self { token, http_client, ws_client }
    }

    pub fn get_identify_string(&self) -> String {
        let event = GatewayEvent {
            opcode: 2,
            data: Some(Data::Identify {
                token: self.token.clone(),
                properties: Properties {
                    os: "windows".into(),
                    browser: "disco".into(),
                    device: "disco".into(),
                },
                intents: 513,
            }),
            sequence: None,
            name: None
        };

        serde_json::to_string(&event).unwrap()
    }

    pub fn post(&self, url: String) -> awc::ClientRequest {
        self.http_client.post(format!("{API_BASE}{url}"))
            .insert_header(("Authorization", format!("Bot {}", self.token)))
            .insert_header(("Content-Type", "application/json"))
    }
}
