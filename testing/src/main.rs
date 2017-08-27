extern crate lavalink;

use lavalink::*;

use std::collections::HashMap;

fn main() {
    // creating a lavalink client
    let client = Client::new("testing-user", 1, 5000);

    // adding a lavalink node
    let mut node_manager = client.node_manager;
    node_manager.add_node("0.0.0.0:25565".to_string(), "lolpasswordxd".to_string());

    // getting the Node instance
    let nodes = node_manager.get_nodes();
    let node = nodes.iter()
        .find(|n| n.server_uri == "0.0.0.0:25565".to_string())
        .unwrap();

    // creating a new Player
    let mut player: Player<Listener> = Player::new("272410239947767808".to_owned());

    // registering a listener - in a new scope so player can be borrowed mutably and returned
    // before inserted into players
    {
        let listener = Listener {};
        let mut listener_manager = &mut player.player_listener_manager;
        listener_manager.add_listener(listener);
    }

    // adding the player to the PlayerManager
    let mut players = HashMap::new();
    players.insert("272410239947767808", &player);

    let socket: Socket<Listener> = Socket::new();
    socket.run();
}

struct Listener;

impl PlayerListener for Listener {
    fn on_player_pause(guild_id: String) {
        println!("Player paused for guild {}!", guild_id);
    }

    fn on_player_resume(guild_id: String) {
        println!("Player resumed for guild {}!", guild_id);
    }

    fn on_track_start(guild_id: String, track: String) {
        println!("Track {} started for guild {}!", track, guild_id);
    }

    fn on_track_end(guild_id: String, track: String, end_reason: String) {
        println!("Track {} ended for guild {} with reason {}!", track, guild_id, end_reason);
    }

    fn on_track_exception(guild_id: String, track: String, exception: String) {
        println!("Track {} threw an exception for guild {}:\n{}", track, guild_id, exception);
    }

    fn on_track_stuck(guild_id: String, track: String, threshold_ms: i64) {
        println!("Track {} got stuck for guild {} with threshold (ms): {}", track, guild_id,
                 threshold_ms);
    }
}
