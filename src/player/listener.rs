pub trait PlayerListener {
    fn on_player_pause(guild_id: String);

    fn on_player_resume(guild_id: String);

    fn on_track_start(guild_id: String, track: String);

    fn on_track_end(guild_id: String, track: String, end_reason: String);

    fn on_track_exception(guild_id: String, track: String, exception: String);

    fn on_track_stuck(guild_id: String, track: String, threshold_ms: i64);
}

pub struct PlayerListenerManager<T: PlayerListener + 'static> {
    listeners: Vec<T>,
}

impl<T: PlayerListener + 'static> PlayerListenerManager<T> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: T) -> usize {
        self.listeners.push(listener);
        self.listeners.len()
    }

    pub fn remove_listener(&mut self, index: usize) -> T {
        self.listeners.remove(index)
    }
}