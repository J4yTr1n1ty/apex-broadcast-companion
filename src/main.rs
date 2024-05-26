use std::{net::TcpListener, thread::spawn};

use prost::Message as WsMessage;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

pub mod events {
    tonic::include_proto!("rtech.liveapi");
}

pub mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:7777").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read().unwrap();

                let binary = msg.into_data();

                let mut bytes = util::bytevector::ByteVector { inner: binary };

                let content = events::LiveApiEvent::decode(&mut bytes)
                    .expect("Unable to decode incoming Message");
                println!("{}", content.game_message.unwrap().type_url);
            }
        });
    }

    Ok(())
}
