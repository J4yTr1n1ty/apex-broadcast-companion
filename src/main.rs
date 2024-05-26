use std::{net::TcpListener, thread::spawn};

use events::Init;
use prost::Message as WsMessage;
use prost_types::Any;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

use crate::util::registry;

pub mod events {
    tonic::include_proto!("rtech.liveapi");
}

pub mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:7777").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |_req: &Request, response: Response| Ok(response);
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read().unwrap();

                let binary = msg.into_data();

                let mut bytes = util::bytevector::ByteVector { inner: binary };

                let content = events::LiveApiEvent::decode(&mut bytes)
                    .expect("Unable to decode incoming Message");

                let event_identifier = content.game_message.unwrap().type_url;

                println!("Received {}", event_identifier);

                let registry = registry::get_registry();
                if let Some(deserializer) = registry.get(&event_identifier) {
                    let event = deserializer(&bytes.inner).expect("Unable to deserialize Message");
                    if let Some(init_event) = event.downcast_ref::<Init>() {
                        println!(
                            "Deserialized Init Event: Timestamp {} Game Version {}",
                            init_event.timestamp, init_event.game_version
                        );
                    }
                } else {
                    eprintln!("Cannot handle event {}", &event_identifier)
                }
            }
        });
    }

    Ok(())
}
