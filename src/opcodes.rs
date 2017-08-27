use std::string::ToString;

pub enum Opcode {
    // Make the server queue a voice connection
    Connect,

    // Provide an intercepted voice server update
    VoiceUpdate,

    // Close a voice connection
    Disconnect,

    // Request to check if the VC or Guild exists, and that we have access to the VC
    ValidationRequest,

    // Response to ValidationRequest
    ValidationResponse,

    // Request to check if a shard's mainWS is connected
    IsConnectedRequest,

    // Response to IsConnectedRequest
    IsConnectedResponse,

    // Cause the player to play a track
    Play,

    // Cause the player to stop
    Stop,

    // Set player pause
    Pause,

    // Make the player seek to a position of the track
    Seek,

    // Set player volume
    Volume,

    // Incoming message to forward to mainWS
    SendWS,

    // Position information about a player
    PlayerUpdate,

    // A collection of stats sent every minute
    Stats,

    // Server emitted an event
    Event,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        use Opcode::*;

        match *self {
            Connect => "connect",
            VoiceUpdate => "voiceUpdate",
            Disconnect => "disconnect",
            ValidationRequest => "validationReq",
            ValidationResponse => "validationRes",
            IsConnectedRequest => "isConnectedReq",
            IsConnectedResponse => "isConnectedRes",
            Play => "play",
            Stop => "stop",
            Pause => "pause",
            Seek => "seek",
            Volume => "volume",
            SendWS => "sendWS",
            PlayerUpdate => "playerUpdate",
            Stats => "stats",
            Event => "event",
        }.to_string()
    }
}

impl Opcode {
    pub fn from_string(string: String) -> Self {
        use Opcode::*;

        match string {
            "connect" => Connect,
            "voiceUpdate" => VoiceUpdate,
            "disconnect" => Disconnect,
            "validationReq" => ValidationRequest,
            "validationRes" => ValidationResponse,
            "isConnectedReq" => IsConnectedRequest,
            "isConnectedRes" => IsConnectedResponse,
            "play" => Play,
            "stop" => Stop,
            "pause" => Pause,
            "seek" => Seek,
            "volume" => Volume,
            "sendWS" => SendWS,
            "playerUpdate" => PlayerUpdate,
            "stats" => Stats,
            "event" => Event,
        }
    }
}