mod listener;
mod manager;
mod state;

use super::*;
pub use self::listener::*;
pub use self::manager::*;
pub use self::state::*;

pub struct Player<T: PlayerListener + 'static> {
    socket: Socket,
    guild_id: String,
    player_listener_manager: PlayerListenerManager<T>,
    player_state: PlayerState,
}

impl<T: PlayerListener + 'static> Player<T> {
    pub fn new(socket: Socket, guild_id: String) -> Self {
        Self {
            socket,
            guild_id,
            player_listener_manager: PlayerListenerManager::new(),
            player_state: PlayerState::new(),
        }
    }

    pub fn set_socket(&mut self, socket: Socket) {
        unimplemented!()
    }

    pub fn play_track(&self, track: String) {
        unimplemented!()
    }

    pub fn stop_track(&self) {
        unimplemented!()
    }

    pub fn is_paused(&self) -> bool {
        self.player_state.paused
    }

    pub fn set_paused(&self, paused: bool) {
        unimplemented!()
    }

    pub fn get_position(&self) -> i64 {
        self.player_state.position
    }

    pub fn set_position(&self, position: i64) {
        unimplemented!()
    }

    pub fn get_volume(&self) -> i32 {
        self.player_state.volume
    }

    pub fn set_volume(&self, volume: i32) {
        unimplemented!()
    }

    pub fn clear_track(&mut self) {
        self.player_state.track = None;
    }
}