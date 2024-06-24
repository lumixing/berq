use std::{thread, time::{Duration, Instant}};
use awc::ws;
use client::Client;
use event::{get_heartbeat_string, Data, GatewayEvent};
use futures_util::{SinkExt as _, StreamExt as _};
use message::Message;
use tokio::{select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

mod client;
mod event;
mod channel;
mod message;
mod user;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting ws client...");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<String>();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    let token = std::env::var("BOT_TOKEN").expect("expected BOT_TOKEN");
    let client = Client::new(token);

    let (_res, mut ws) = client.ws_client
        .ws("wss://gateway.discord.gg")
        .connect().await.unwrap();

    log::info!("connected!");

    log::info!("=> sending identify");
    ws.send(ws::Message::Text(client.get_identify_string().into())).await.unwrap();

    let mut prev_heartbeat = Instant::now();
    let heartbeat_duration = Duration::from_millis(41250); // bad!

    let input_thread = thread::spawn(move || loop {
        if prev_heartbeat.elapsed() >= heartbeat_duration {
            prev_heartbeat = Instant::now();

            log::info!("=> sending heartbeat");
            cmd_tx.send(get_heartbeat_string()).unwrap();
        }
    });

    loop {
        select! {
            Some(msg) = ws.next() => {
                if let Ok(ws::Frame::Text(txt)) = msg {
                    let txtstr = std::str::from_utf8(&txt).unwrap();
                    let event: GatewayEvent = serde_json::from_str(txtstr).unwrap();
                    log::info!("=> {} {}", event.opcode, event.name.unwrap_or("".into()));

                    match event.data {
                        Some(event) => {
                            match event {
                                Data::MessageCreate { message } => {
                                    handle(&client, message).await;
                                }
                                _ => {}
                            }
                        }
                        None => { // TERRIFIC
                            // log::info!("<= heartbeat ack");
                        }
                    }
                }
            }

            Some(cmd) = cmd_rx.next() => {
                if cmd.is_empty() { continue }
                ws.send(ws::Message::Text(cmd.into())).await.unwrap();
            }

            else => break
        }
    }

    input_thread.join().unwrap();
}

async fn handle(client: &Client, message: Message) {
    if message.author.is_bot() { return }
    if !message.content.starts_with(".") { return }

    let mut args: Vec<&str> = message.content.split_whitespace().collect();
    let cmd = args.remove(0);
    // log::info!("{cmd:?} {args:?}");
    if cmd == ".ping" {
        message.get_channel().create_reply(&client, "pong!", message).await;
    }
}
