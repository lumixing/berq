use serde::{Deserialize, Serialize};

use crate::message::Message;

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Opcode {
//     Dispatch            = 0,
//     Heartbeat           = 1,
//     Identity            = 2,
//     PresenceUpdate      = 3,
//     VoiceStateUpdate    = 4,
//     Resume              = 6,
//     Reconnect           = 7,
//     RequestGuildMembers = 8,
//     InvalidSession      = 9,
//     Hello               = 10,
//     HeartbeatACK        = 11,
// }

// impl Opcode {
//     fn from_int(int: i32) -> Self {
//         match int {
//             0 => Self::Dispatch,
//             1 => Self::Heartbeat,
//             2 => Self::Identity,
//             3 => Self::PresenceUpdate,
//             4 => Self::VoiceStateUpdate,
//             6 => Self::Resume,
//             7 => Self::Reconnect,
//             8 => Self::RequestGuildMembers,
//             9 => Self::InvalidSession,
//             10 => Self::Hello,
//             11 => Self::HeartbeatACK,
//             n => panic!("Could not convert {n} to a Gateway Opcode")
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayEvent {
    #[serde(rename = "op")]
    pub opcode: i32,

    #[serde(rename = "d")]
    pub data: Option<Data>,

    #[serde(rename = "s")]
    pub sequence: Option<i32>,

    #[serde(rename = "t")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Data {
    Hello {
        heartbeat_interval: i32,
    },
    Ready {
        #[serde(rename = "v")]
        version: i32,
    },
    GuildCreate {
        id: String,
        name: String,
    },
    MessageCreate {
        #[serde(flatten)]
        message: Message,
    },
    Identify {
        token: String,
        properties: Properties,
        intents: i32,
    },
    Sequence(i32)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "$os")]
    pub os: String,

    #[serde(rename = "$browser")]
    pub browser: String,

    #[serde(rename = "$device")]
    pub device: String,
}

pub fn get_heartbeat_string() -> String {
    let event = GatewayEvent {
        opcode: 1,
        data: Some(Data::Sequence(1)),
        sequence: None,
        name: None
    };

    return serde_json::to_string(&event).unwrap()
}
