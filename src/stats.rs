// todo serde
pub struct FrameStats {
    // average frames sent per minute
    sent: i32,
    // average frames nulled per minute
    nulled: i32,
    // average frames deficit per minute
    deficit: i32,
}

pub struct RemoteStats {
    pub players: i32,
    pub playing_players: i32,
    pub uptime: i64,

    pub mem_free: i32,
    pub mem_used: i32,
    pub mem_allocated: i32,
    pub mem_reservable: i32,

    pub cpu_cores: i32,
    pub system_load: i64,
    pub lavalink_load: i64,

    pub frame_stats: Option<FrameStats>,
}