//! HEOS type definitions
//!
//! Contains all the types used for HEOS API communication and responses.

use serde::{Deserialize, Serialize};

/// HEOS connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeosConfig {
    pub host: String,
    pub port: u16,
    pub player_id: i64,
}

impl Default for HeosConfig {
    fn default() -> Self {
        Self {
            host: "192.168.1.2".to_string(),
            port: 1255,
            player_id: 0,
        }
    }
}

/// Generic HEOS response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeosResponse<T> {
    pub heos: HeosHeader,
    #[serde(default)]
    pub payload: Option<T>,
}

/// HEOS response header (present in all responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeosHeader {
    pub command: String,
    pub result: String,
    #[serde(default)]
    pub message: String,
}

impl HeosHeader {
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }

    /// Parse message string into key-value pairs
    /// Format: "pid=123&volume=50&mute=off"
    pub fn parse_message(&self) -> std::collections::HashMap<String, String> {
        self.message
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                    _ => None,
                }
            })
            .collect()
    }
}

// ============================================================================
// Player Types
// ============================================================================

/// Player information returned by get_players
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub pid: i64,
    pub model: String,
    pub version: String,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub lineout: Option<u8>,
    #[serde(default)]
    pub serial: Option<String>,
}

/// Play state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayState {
    Play,
    Pause,
    Stop,
}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayState::Play => write!(f, "play"),
            PlayState::Pause => write!(f, "pause"),
            PlayState::Stop => write!(f, "stop"),
        }
    }
}

impl std::str::FromStr for PlayState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "play" => Ok(PlayState::Play),
            "pause" => Ok(PlayState::Pause),
            "stop" => Ok(PlayState::Stop),
            _ => Err(format!("Unknown play state: {}", s)),
        }
    }
}

/// Mute state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuteState {
    On,
    Off,
}

impl std::fmt::Display for MuteState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MuteState::On => write!(f, "on"),
            MuteState::Off => write!(f, "off"),
        }
    }
}

impl std::str::FromStr for MuteState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "on" => Ok(MuteState::On),
            "off" => Ok(MuteState::Off),
            _ => Err(format!("Unknown mute state: {}", s)),
        }
    }
}

/// Repeat mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatMode {
    Off,
    One,
    All,
}

impl std::fmt::Display for RepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepeatMode::Off => write!(f, "off"),
            RepeatMode::One => write!(f, "on_one"),
            RepeatMode::All => write!(f, "on_all"),
        }
    }
}

/// Shuffle mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShuffleMode {
    On,
    Off,
}

impl std::fmt::Display for ShuffleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShuffleMode::On => write!(f, "on"),
            ShuffleMode::Off => write!(f, "off"),
        }
    }
}

// ============================================================================
// Now Playing Types
// ============================================================================

/// Now playing media information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NowPlayingMedia {
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub song: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub image_url: Option<String>,
    pub mid: Option<String>,
    pub qid: Option<i64>,
    pub sid: Option<i64>,
    pub station: Option<String>,
}

// ============================================================================
// Browse/Source Types
// ============================================================================

/// Music source information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusicSource {
    pub name: String,
    pub image_url: Option<String>,
    #[serde(rename = "type")]
    pub source_type: String,
    pub sid: i64,
    #[serde(default)]
    pub available: Option<String>,
    #[serde(default)]
    pub service_username: Option<String>,
}

/// Known source IDs
pub mod source_ids {
    pub const PANDORA: i64 = 1;
    pub const RHAPSODY: i64 = 2;
    pub const TUNEIN: i64 = 3;
    pub const SPOTIFY: i64 = 4;
    pub const DEEZER: i64 = 5;
    pub const NAPSTER: i64 = 6;
    pub const IHEARTRADIO: i64 = 7;
    pub const SIRIUS_XM: i64 = 8;
    pub const SOUNDCLOUD: i64 = 9;
    pub const TIDAL: i64 = 10;
    pub const AMAZON_MUSIC: i64 = 13;
    pub const LOCAL_MUSIC: i64 = 1024;
    pub const PLAYLISTS: i64 = 1025;
    pub const HISTORY: i64 = 1026;
    pub const AUX_INPUT: i64 = 1027;
    pub const FAVORITES: i64 = 1028;
}

/// Input source names (for play_input command)
pub mod inputs {
    pub const AUX_IN_1: &str = "inputs/aux_in_1";
    pub const AUX_IN_2: &str = "inputs/aux_in_2";
    pub const AUX_IN_3: &str = "inputs/aux_in_3";
    pub const AUX_IN_4: &str = "inputs/aux_in_4";
    pub const OPTICAL_IN_1: &str = "inputs/optical_in_1";
    pub const OPTICAL_IN_2: &str = "inputs/optical_in_2";
    pub const COAX_IN_1: &str = "inputs/coax_in_1";
    pub const COAX_IN_2: &str = "inputs/coax_in_2";
    pub const HDMI_IN_1: &str = "inputs/hdmi_in_1";
    pub const HDMI_IN_2: &str = "inputs/hdmi_in_2";
    pub const HDMI_IN_3: &str = "inputs/hdmi_in_3";
    pub const HDMI_IN_4: &str = "inputs/hdmi_in_4";
    pub const HDMI_ARC_1: &str = "inputs/hdmi_arc_1";
    pub const CABLE_SAT: &str = "inputs/cable_sat";
    pub const DVD: &str = "inputs/dvd";
    pub const BLURAY: &str = "inputs/bluray";
    pub const GAME: &str = "inputs/game";
    pub const MEDIA_PLAYER: &str = "inputs/mediaplayer";
    pub const CD: &str = "inputs/cd";
    pub const TUNER: &str = "inputs/tuner";
    pub const TV_AUDIO: &str = "inputs/tvaudio";
    pub const PHONO: &str = "inputs/phono";
}

/// Add to queue criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddCriteria {
    PlayNow = 1,
    PlayNext = 2,
    AddToEnd = 3,
    ReplaceAndPlay = 4,
}

// ============================================================================
// Event Types (for change events)
// ============================================================================

/// HEOS change event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum HeosEvent {
    #[serde(rename = "event/player_state_changed")]
    PlayerStateChanged { pid: i64, state: String },

    #[serde(rename = "event/player_now_playing_changed")]
    NowPlayingChanged { pid: i64 },

    #[serde(rename = "event/player_now_playing_progress")]
    NowPlayingProgress {
        pid: i64,
        cur_pos: i64,
        duration: i64,
    },

    #[serde(rename = "event/player_volume_changed")]
    VolumeChanged { pid: i64, level: u8, mute: String },

    #[serde(rename = "event/player_queue_changed")]
    QueueChanged { pid: i64 },

    #[serde(rename = "event/players_changed")]
    PlayersChanged,

    #[serde(rename = "event/groups_changed")]
    GroupsChanged,

    #[serde(rename = "event/sources_changed")]
    SourcesChanged,

    #[serde(rename = "event/player_playback_error")]
    PlaybackError { pid: i64, error: String },
}

// ============================================================================
// Error Types
// ============================================================================

/// HEOS error codes
#[derive(Debug, Clone)]
pub struct HeosError {
    pub code: i32,
    pub message: String,
}

impl std::fmt::Display for HeosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HEOS Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for HeosError {}

/// Known error codes from HEOS documentation
pub mod error_codes {
    pub const UNRECOGNIZED_COMMAND: i32 = 1;
    pub const INVALID_ID: i32 = 2;
    pub const WRONG_ARGUMENTS: i32 = 3;
    pub const RESOURCE_NOT_FOUND: i32 = 4;
    pub const RESOURCE_IN_USE: i32 = 5;
    pub const INVALID_CREDENTIALS: i32 = 6;
    pub const USER_NOT_LOGGED_IN: i32 = 7;
    pub const USER_NOT_FOUND: i32 = 8;
    pub const INTERNAL_ERROR: i32 = 9;
    pub const SYSTEM_ERROR: i32 = 10;
    pub const PROCESSING_ERROR: i32 = 12;
    pub const MEDIA_NOT_FOUND: i32 = 13;
    pub const OPTION_NOT_SUPPORTED: i32 = 14;
    pub const TOO_MANY_REQUESTS: i32 = 15;
    pub const COMMAND_NOT_PROCESSED: i32 = 16;
}
