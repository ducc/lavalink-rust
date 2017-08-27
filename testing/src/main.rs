extern crate lavalink;

use lavalink::*;

fn main() {
    // creating a lavalink client
    let client = Client::new("testing-user", 1, 5000);

    // adding a lavalink node
    let mut node_manager = client.node_manager;
    node_manager.add_node("0.0.0.0:25565".to_string(), "lolpasswordxd".to_string());

    // creating a new Player
    let player: Player<Listener> = Player::new("272410239947767808".to_owned());

    // registering a listener
    let listener = Listener {};
    let mut listener_manager = player.player_listener_manager;
    listener_manager.add_listener(listener);

    let socket = Socket::new();
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
