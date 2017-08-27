extern crate serde_json;

use ::node::*;
use ::opcodes::*;
use ::stats::*;

use std::thread;
use std::sync::mpsc::channel;
use std::io::stdin;
use std::str::FromStr;

use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;
use websocket::header::Headers;

use serde_json::{Value, Error};

pub struct Socket {

}

impl Socket {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn run(&self) {
        let mut headers = Headers::new();
        headers.set_raw("Authorization", vec![b"password".to_vec()]);
        headers.set_raw("Num-Shards", vec![b"1".to_vec()]);
        headers.set_raw("User-Id", vec![b"test-user-id".to_vec()]);

        let client = ClientBuilder::new("ws://localhost:8012")
            .unwrap()
            .add_protocol("rust-websocket")
            .custom_headers(&headers)
            .connect_insecure()
            .unwrap();

        println!("Connected!");

        let (mut receiver, mut sender) = client.split().unwrap();
        let (tx, rx) = channel();
        let tx_1 = tx.clone();

        // tbis loop waits for a new message in the rx channel. once the message has been received
        // it sends it to the websocket via the client's sender
        let send_loop = thread::spawn(move || {
            loop {
                // send loop
                let message = match rx.recv() { // blocking call, waits for a message in rx channel
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send loop: {:?}", e);
                        return;
                    }
                };

                match message {
                    // if it is a close message send the message and then return; to exit the loop
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        return;
                    },
                    // otherwise continue
                    _ => (),
                }

                // send the message
                match sender.send_message(&message) {
                    Ok(()) => (), // message was sent successfully
                    Err(e) => { // oh no an error occurred when sending the message
                        println!("Send loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });

        // this loop waits for a new message to from the websocket server
        let receive_loop = thread::spawn(move || {
            // receiver.incoming_messages() is a blocking call that waits for the next message
            for message in receiver.incoming_messages() {
                let message = match message {
                    // woo message came in and its not broken!!
                    Ok(m) => m,
                    // oopsie that message is fucked lmao, send a close message into the sending
                    // channel and then exit out of the loop to stop the thread execution
                    Err(e) => {
                        println!("Receive loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };

                match message {
                    // the server sent a close message so send a close message into the sending
                    // channel to kill the sending thread, then return to exit out of the loop and
                    // stop the thread execution
                    OwnedMessage::Close(_) => {
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    },
                    // hehe the server sent a ping :) lets send a pong to the sending channel in
                    // response my dude
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            Ok(()) => (), // ponged well
                            Err(e) => { // ponged badly and had an error, exit loop!?!>!?
                                println!("Receive loop: {:?}", e);
                                return;
                            }
                        }
                    },
                    // text msg!!!!!!!!
                    OwnedMessage::Text(data) => {
                        println!("Receive loop text message: {}", data);
                        //self.handle_message(data.clone());

                        Socket::handle_message(data);
                    },
                    // received something else?
                    _ => {
                        println!("Receive loop: {:?}", message)
                    }
                }
            }
        });

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let trimmed = input.trim();

            let message = match trimmed {
                "/close" => {
                    // close the connection
                    let _ = tx.send(OwnedMessage::Close(None));
                    break;
                },
                "/ping" => {
                    // send a ping
                    OwnedMessage::Ping(b"PING".to_vec())
                },
                _ => {
                    // just send text
                    OwnedMessage::Text(trimmed.to_string())
                }
            };

            match tx.send(message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Main loop: {:?}", e);
                    break;
                }
            }
        }

        // exiting
        println!("waiting for child threads to exit");

        let _ = send_loop.join();
        let _ = receive_loop.join();

        println!("goodbye my dude");
    }

    pub fn handle_message(text: String) {
        let json: Value = serde_json::from_str(text.as_ref()).unwrap();
        let op = json["op"].as_str().unwrap();
        let opcode = Opcode::from_str(op).unwrap();

        use Opcode::*;

        match opcode {
            Stats => {
                let stats = RemoteStats::from_json(&json);
                println!("Stats = {:?}", stats);
            },
            _ => {},
        }
    }
}

pub struct SocketManager {

}

impl SocketManager {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn connect(&self) {
        unimplemented!()
    }

    pub fn disconnect(&self) {
        unimplemented!()
    }

    // get_socket(guild: Guild) -> Socket
    pub fn get_socket(&self) {
        unimplemented!()
    }
}