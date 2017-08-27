mod listener;
mod state;

use super::socket::*;

pub use self::listener::*;
pub use self::state::*;

pub struct Player<T: PlayerListener + 'static> {
    guild_id: String,
    pub player_listener_manager: PlayerListenerManager<T>,
    player_state: PlayerState,
}

impl<T: PlayerListener + 'static> Player<T> {
    pub fn new(guild_id: String) -> Self {
        Self {
            guild_id,
            player_listener_manager: PlayerListenerManager::new(),
            player_state: PlayerState::new(),
        }
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