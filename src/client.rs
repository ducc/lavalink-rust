use ::*;

pub struct Client {
    pub user_id: String,
    pub num_shards: i32,
    pub reconnect_timeout: i32,
    pub socket_manager: SocketManager,
    pub node_manager: NodeManager,
    pub audio_manager: AudioManager,
    pub player_manager: PlayerManager,
}

impl<'a> Client {
    pub fn new(user_id: &'a str, num_shards: i32, reconnect_timeout: i32) -> Self {
        Client {
            user_id: user_id.to_string(),
            num_shards,
            reconnect_timeout,
            socket_manager: SocketManager::new(),
            node_manager: NodeManager::new(),
            audio_manager: AudioManager::new(),
            player_manager: PlayerManager::new(),
        }
    }

    pub fn shutdown(&self) {
        unimplemented!()
    }
}