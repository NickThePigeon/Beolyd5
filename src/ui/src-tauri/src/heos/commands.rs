//! HEOS Command implementations
//!
//! All HEOS CLI commands organized by category:
//! - System commands (heartbeat, register events)
//! - Player commands (volume, play state, now playing)
//! - Browse commands (sources, inputs)

use super::client::HeosClient;
use super::types::*;

/// Maximum volume limit for safety (protect speakers)
/// Set to 50% to prevent accidental loud volume
pub const MAX_VOLUME_LIMIT: u8 = 75;

// ============================================================================
// System Commands
// ============================================================================

impl HeosClient {
    /// Send heartbeat to check connection
    /// Command: heos://system/heart_beat
    pub fn heartbeat(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_command_simple("heos://system/heart_beat")?;
        Ok(())
    }

    /// Register for change events
    /// Command: heos://system/register_for_change_events?enable=on|off
    pub fn register_for_change_events(
        &self,
        enable: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = if enable { "on" } else { "off" };
        let cmd = format!("heos://system/register_for_change_events?enable={}", state);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Enable pretty JSON responses (useful for debugging)
    /// Command: heos://system/prettify_json_response?enable=on|off
    pub fn prettify_json(&self, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        let state = if enable { "on" } else { "off" };
        let cmd = format!("heos://system/prettify_json_response?enable={}", state);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Reboot the HEOS speaker
    /// Command: heos://system/reboot
    pub fn reboot(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_command_simple("heos://system/reboot")?;
        Ok(())
    }
}

// ============================================================================
// Player Commands
// ============================================================================

impl HeosClient {
    /// Get list of all players on the network
    /// Command: heos://player/get_players
    pub fn get_players(&self) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
        let response: HeosResponse<Vec<Player>> =
            self.send_command_parsed("heos://player/get_players")?;
        Ok(response.payload.unwrap_or_default())
    }

    /// Get info for a specific player
    /// Command: heos://player/get_player_info?pid=<player_id>
    pub fn get_player_info(&self, pid: i64) -> Result<Player, Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_player_info?pid={}", pid);
        let response: HeosResponse<Player> = self.send_command_parsed(&cmd)?;
        response
            .payload
            .ok_or_else(|| "No player info returned".into())
    }

    /// Get current play state
    /// Command: heos://player/get_play_state?pid=<player_id>
    pub fn get_play_state(&self, pid: i64) -> Result<PlayState, Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_play_state?pid={}", pid);
        let header = self.send_command_simple(&cmd)?;
        let msg = header.parse_message();
        let state_str = msg.get("state").ok_or("No state in response")?;
        state_str.parse().map_err(|e: String| e.into())
    }

    /// Get play state for configured player
    pub fn get_play_state_current(&self) -> Result<PlayState, Box<dyn std::error::Error>> {
        self.get_play_state(self.player_id())
    }

    /// Set play state (play, pause, stop)
    /// Command: heos://player/set_play_state?pid=<player_id>&state=<play|pause|stop>
    pub fn set_play_state(
        &self,
        pid: i64,
        state: PlayState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/set_play_state?pid={}&state={}", pid, state);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Set play state for configured player
    pub fn set_play_state_current(
        &self,
        state: PlayState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_play_state(self.player_id(), state)
    }

    /// Convenience: Play
    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_play_state_current(PlayState::Play)
    }

    /// Convenience: Pause
    pub fn pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_play_state_current(PlayState::Pause)
    }

    /// Convenience: Stop
    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_play_state_current(PlayState::Stop)
    }

    /// Toggle play/pause
    pub fn toggle_play_pause(&self) -> Result<PlayState, Box<dyn std::error::Error>> {
        let current = self.get_play_state_current()?;
        let new_state = match current {
            PlayState::Play => PlayState::Pause,
            PlayState::Pause | PlayState::Stop => PlayState::Play,
        };
        self.set_play_state_current(new_state)?;
        Ok(new_state)
    }

    /// Get now playing media info
    /// Command: heos://player/get_now_playing_media?pid=<player_id>
    pub fn get_now_playing(&self, pid: i64) -> Result<NowPlayingMedia, Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_now_playing_media?pid={}", pid);
        let response: HeosResponse<NowPlayingMedia> = self.send_command_parsed(&cmd)?;
        Ok(response.payload.unwrap_or_default())
    }

    /// Get now playing for configured player
    pub fn get_now_playing_current(&self) -> Result<NowPlayingMedia, Box<dyn std::error::Error>> {
        self.get_now_playing(self.player_id())
    }

    /// Get current volume (0-100)
    /// Command: heos://player/get_volume?pid=<player_id>
    pub fn get_volume(&self, pid: i64) -> Result<u8, Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_volume?pid={}", pid);
        let header = self.send_command_simple(&cmd)?;
        let msg = header.parse_message();
        let level_str = msg.get("level").ok_or("No level in response")?;
        level_str.parse().map_err(|_| "Invalid volume level".into())
    }

    /// Get volume for configured player
    pub fn get_volume_current(&self) -> Result<u8, Box<dyn std::error::Error>> {
        self.get_volume(self.player_id())
    }

    /// Set volume (0-100, clamped to MAX_VOLUME_LIMIT for safety)
    /// Command: heos://player/set_volume?pid=<player_id>&level=<0-100>
    pub fn set_volume(&self, pid: i64, level: u8) -> Result<(), Box<dyn std::error::Error>> {
        let level = level.min(MAX_VOLUME_LIMIT); // Clamp to safety limit
        let cmd = format!("heos://player/set_volume?pid={}&level={}", pid, level);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Set volume for configured player
    pub fn set_volume_current(&self, level: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.set_volume(self.player_id(), level)
    }

    /// Volume up by step (default 5), respects MAX_VOLUME_LIMIT
    /// Command: heos://player/volume_up?pid=<player_id>&step=<1-10>
    pub fn volume_up(&self, pid: i64, step: u8) -> Result<(), Box<dyn std::error::Error>> {
        // First check current volume to respect safety limit
        let current = self.get_volume(pid)?;
        if current >= MAX_VOLUME_LIMIT {
            return Ok(()); // Already at or above limit
        }
        let step = step.min(10).max(1); // Clamp 1-10
                                        // Calculate safe step to not exceed limit
        let safe_step = step.min(MAX_VOLUME_LIMIT.saturating_sub(current));
        if safe_step == 0 {
            return Ok(());
        }
        let cmd = format!("heos://player/volume_up?pid={}&step={}", pid, safe_step);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Volume up for configured player
    pub fn volume_up_current(&self, step: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.volume_up(self.player_id(), step)
    }

    /// Volume down by step (default 5)
    /// Command: heos://player/volume_down?pid=<player_id>&step=<1-10>
    pub fn volume_down(&self, pid: i64, step: u8) -> Result<(), Box<dyn std::error::Error>> {
        let step = step.min(10).max(1); // Clamp 1-10
        let cmd = format!("heos://player/volume_down?pid={}&step={}", pid, step);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Volume down for configured player
    pub fn volume_down_current(&self, step: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.volume_down(self.player_id(), step)
    }

    /// Get mute state
    /// Command: heos://player/get_mute?pid=<player_id>
    pub fn get_mute(&self, pid: i64) -> Result<MuteState, Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_mute?pid={}", pid);
        let header = self.send_command_simple(&cmd)?;
        let msg = header.parse_message();
        let state_str = msg.get("state").ok_or("No state in response")?;
        state_str.parse().map_err(|e: String| e.into())
    }

    /// Get mute for configured player
    pub fn get_mute_current(&self) -> Result<MuteState, Box<dyn std::error::Error>> {
        self.get_mute(self.player_id())
    }

    /// Set mute state
    /// Command: heos://player/set_mute?pid=<player_id>&state=on|off
    pub fn set_mute(&self, pid: i64, state: MuteState) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/set_mute?pid={}&state={}", pid, state);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Set mute for configured player
    pub fn set_mute_current(&self, state: MuteState) -> Result<(), Box<dyn std::error::Error>> {
        self.set_mute(self.player_id(), state)
    }

    /// Toggle mute
    /// Command: heos://player/toggle_mute?pid=<player_id>
    pub fn toggle_mute(&self, pid: i64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/toggle_mute?pid={}", pid);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Toggle mute for configured player
    pub fn toggle_mute_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.toggle_mute(self.player_id())
    }

    /// Play next track
    /// Command: heos://player/play_next?pid=<player_id>
    pub fn play_next(&self, pid: i64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/play_next?pid={}", pid);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Play next for configured player
    pub fn play_next_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.play_next(self.player_id())
    }

    /// Play previous track
    /// Command: heos://player/play_previous?pid=<player_id>
    pub fn play_previous(&self, pid: i64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/play_previous?pid={}", pid);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Play previous for configured player
    pub fn play_previous_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.play_previous(self.player_id())
    }

    /// Get play mode (repeat/shuffle)
    /// Command: heos://player/get_play_mode?pid=<player_id>
    pub fn get_play_mode(
        &self,
        pid: i64,
    ) -> Result<(RepeatMode, ShuffleMode), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/get_play_mode?pid={}", pid);
        let header = self.send_command_simple(&cmd)?;
        let msg = header.parse_message();

        let repeat = match msg.get("repeat").map(|s| s.as_str()) {
            Some("on_one") => RepeatMode::One,
            Some("on_all") => RepeatMode::All,
            _ => RepeatMode::Off,
        };

        let shuffle = match msg.get("shuffle").map(|s| s.as_str()) {
            Some("on") => ShuffleMode::On,
            _ => ShuffleMode::Off,
        };

        Ok((repeat, shuffle))
    }

    /// Set play mode
    /// Command: heos://player/set_play_mode?pid=<player_id>&repeat=<off|on_one|on_all>&shuffle=<on|off>
    pub fn set_play_mode(
        &self,
        pid: i64,
        repeat: RepeatMode,
        shuffle: ShuffleMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!(
            "heos://player/set_play_mode?pid={}&repeat={}&shuffle={}",
            pid, repeat, shuffle
        );
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Clear the queue
    /// Command: heos://player/clear_queue?pid=<player_id>
    pub fn clear_queue(&self, pid: i64) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://player/clear_queue?pid={}", pid);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Clear queue for configured player
    pub fn clear_queue_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_queue(self.player_id())
    }
}

// ============================================================================
// Browse Commands
// ============================================================================

impl HeosClient {
    /// Get available music sources
    /// Command: heos://browse/get_music_sources
    pub fn get_music_sources(&self) -> Result<Vec<MusicSource>, Box<dyn std::error::Error>> {
        let response: HeosResponse<Vec<MusicSource>> =
            self.send_command_parsed("heos://browse/get_music_sources")?;
        Ok(response.payload.unwrap_or_default())
    }

    /// Play an input source (aux, optical, hdmi, etc.)
    /// Command: heos://browse/play_input?pid=<player_id>&input=<input_name>
    pub fn play_input(&self, pid: i64, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://browse/play_input?pid={}&input={}", pid, input);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Play input for configured player
    pub fn play_input_current(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.play_input(self.player_id(), input)
    }

    /// Play TV input (optical_in_1)
    pub fn play_tv(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.play_input_current(inputs::OPTICAL_IN_1)
    }

    /// Play a preset station from HEOS Favorites
    /// Command: heos://browse/play_preset?pid=<player_id>&preset=<1-n>
    pub fn play_preset(&self, pid: i64, preset: u8) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://browse/play_preset?pid={}&preset={}", pid, preset);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Play preset for configured player
    pub fn play_preset_current(&self, preset: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.play_preset(self.player_id(), preset)
    }

    /// Play a URL stream
    /// Command: heos://browse/play_stream?pid=<player_id>&url=<url>
    pub fn play_url(&self, pid: i64, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = format!("heos://browse/play_stream?pid={}&url={}", pid, url);
        self.send_command_simple(&cmd)?;
        Ok(())
    }

    /// Play URL for configured player
    pub fn play_url_current(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.play_url(self.player_id(), url)
    }

    /// Browse a music source
    /// Command: heos://browse/browse?sid=<source_id>
    pub fn browse_source(&self, sid: i64) -> Result<String, Box<dyn std::error::Error>> {
        let cmd = format!("heos://browse/browse?sid={}", sid);
        self.send_command(&cmd)
    }

    /// Browse Spotify
    pub fn browse_spotify(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.browse_source(source_ids::SPOTIFY)
    }

    /// Browse HEOS Favorites
    pub fn browse_favorites(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.browse_source(source_ids::FAVORITES)
    }
}
