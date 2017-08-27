pub struct PlayerState {
    pub track: Option<String>, // lavaplayer encoded track in base64
    pub paused: bool,
    pub volume: i32,
    pub update_time: i64,
    pub position: i64,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            track: None,
            paused: false,
            volume: 100,
            update_time: -1,
            position: -1,
        }
    }
}