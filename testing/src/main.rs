extern crate lavalink;

use lavalink::*;

fn main() {
    // creating a lavalink client
    let client = Client::new("testing-user", 1, 5000);

    // adding a lavalink node
    let mut node_manager = client.node_manager;
    node_manager.add_node("0.0.0.0:25565".to_string(), "lolpasswordxd".to_string());

    let socket = Socket::new();
    socket.run();
}