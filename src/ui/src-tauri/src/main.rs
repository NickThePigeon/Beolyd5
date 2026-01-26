// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod heos;
mod hw_controller;

use heos::{HeosClient, HeosConfig, MusicSource, MuteState, NowPlayingMedia, PlayState, Player};
use hw_controller::HWController;
use std::sync::Mutex;
use tauri::{generate_context, Manager, State};

// ============================================================================
// Hardware Controller Commands
// ============================================================================

#[tauri::command]
async fn tick(state: State<'_, Mutex<HWController>>) -> Result<(), String> {
    let controller = state.lock().map_err(|e| e.to_string())?;
    controller.tick().map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// HEOS Commands
// ============================================================================

#[tauri::command]
async fn heos_connect(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.connect().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_set_host(state: State<'_, Mutex<HeosClient>>, host: String) -> Result<(), String> {
    let mut client = state.lock().map_err(|e| e.to_string())?;
    let new_config = HeosConfig {
        host,
        port: 1255,
        player_id: 0,
    };
    client.set_config(new_config);
    Ok(())
}

#[tauri::command]
async fn heos_get_host(state: State<'_, Mutex<HeosClient>>) -> Result<String, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    Ok(client.host().to_string())
}

/// Connect and auto-discover players, selecting the first one (or by name)
#[tauri::command]
async fn heos_connect_and_discover(
    state: State<'_, Mutex<HeosClient>>,
    player_name: Option<String>,
) -> Result<Player, String> {
    let mut client = state.lock().map_err(|e| e.to_string())?;

    // Connect if not already connected
    if !client.is_connected() {
        client.connect().map_err(|e| e.to_string())?;
    }

    // Get all players
    let players = client.get_players().map_err(|e| e.to_string())?;

    if players.is_empty() {
        return Err("No HEOS players found on the network".to_string());
    }

    // Find player by name or use first one
    let player = if let Some(name) = player_name {
        players
            .iter()
            .find(|p| p.name.to_lowercase().contains(&name.to_lowercase()))
            .cloned()
            .ok_or_else(|| format!("Player '{}' not found", name))?
    } else {
        players.into_iter().next().unwrap()
    };

    // Set the player ID
    client.set_player_id(player.pid);

    Ok(player)
}

#[tauri::command]
async fn heos_disconnect(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.disconnect();
    Ok(())
}

#[tauri::command]
async fn heos_heartbeat(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.heartbeat().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_get_players(state: State<'_, Mutex<HeosClient>>) -> Result<Vec<Player>, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.get_players().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_set_player_id(state: State<'_, Mutex<HeosClient>>, pid: i64) -> Result<(), String> {
    let mut client = state.lock().map_err(|e| e.to_string())?;
    client.set_player_id(pid);
    Ok(())
}

#[tauri::command]
async fn heos_get_volume(state: State<'_, Mutex<HeosClient>>) -> Result<u8, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.get_volume_current().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_set_volume(state: State<'_, Mutex<HeosClient>>, level: u8) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.set_volume_current(level).map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_volume_up(state: State<'_, Mutex<HeosClient>>, step: u8) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.volume_up_current(step).map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_volume_down(state: State<'_, Mutex<HeosClient>>, step: u8) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.volume_down_current(step).map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_get_mute(state: State<'_, Mutex<HeosClient>>) -> Result<bool, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    let mute = client.get_mute_current().map_err(|e| e.to_string())?;
    Ok(mute == MuteState::On)
}

#[tauri::command]
async fn heos_set_mute(state: State<'_, Mutex<HeosClient>>, muted: bool) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    let state = if muted { MuteState::On } else { MuteState::Off };
    client.set_mute_current(state).map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_toggle_mute(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.toggle_mute_current().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_get_play_state(state: State<'_, Mutex<HeosClient>>) -> Result<String, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    let play_state = client.get_play_state_current().map_err(|e| e.to_string())?;
    Ok(play_state.to_string())
}

#[tauri::command]
async fn heos_play(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.play().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_pause(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.pause().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_stop(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.stop().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_toggle_play_pause(state: State<'_, Mutex<HeosClient>>) -> Result<String, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    let new_state = client.toggle_play_pause().map_err(|e| e.to_string())?;
    Ok(new_state.to_string())
}

#[tauri::command]
async fn heos_play_next(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.play_next_current().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_play_previous(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.play_previous_current().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_get_now_playing(
    state: State<'_, Mutex<HeosClient>>,
) -> Result<NowPlayingMedia, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.get_now_playing_current().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_get_music_sources(
    state: State<'_, Mutex<HeosClient>>,
) -> Result<Vec<MusicSource>, String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.get_music_sources().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_play_input(state: State<'_, Mutex<HeosClient>>, input: String) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.play_input_current(&input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_play_tv(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.play_tv().map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_play_preset(state: State<'_, Mutex<HeosClient>>, preset: u8) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client
        .play_preset_current(preset)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn heos_clear_queue(state: State<'_, Mutex<HeosClient>>) -> Result<(), String> {
    let client = state.lock().map_err(|e| e.to_string())?;
    client.clear_queue_current().map_err(|e| e.to_string())
}

// ============================================================================
// Main Application
// ============================================================================

fn main() {
    // HEOS configuration - player_id will be set dynamically via heos_connect_and_discover
    let heos_config = HeosConfig {
        host: "192.168.1.2".to_string(), // Default, can be changed via heos_set_host
        port: 1255,
        player_id: 0, // Will be set dynamically
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Hardware controller
            tick,
            // HEOS connection
            heos_connect,
            heos_disconnect,
            heos_heartbeat,
            heos_set_host,
            heos_get_host,
            heos_connect_and_discover,
            // HEOS player discovery
            heos_get_players,
            heos_set_player_id,
            // HEOS volume
            heos_get_volume,
            heos_set_volume,
            heos_volume_up,
            heos_volume_down,
            // HEOS mute
            heos_get_mute,
            heos_set_mute,
            heos_toggle_mute,
            // HEOS playback
            heos_get_play_state,
            heos_play,
            heos_pause,
            heos_stop,
            heos_toggle_play_pause,
            heos_play_next,
            heos_play_previous,
            // HEOS now playing
            heos_get_now_playing,
            // HEOS sources
            heos_get_music_sources,
            heos_play_input,
            heos_play_tv,
            heos_play_preset,
            heos_clear_queue,
        ])
        .setup(|app| {
            let app_handle = app.handle();

            // In dev mode, make the window bigger for easier debugging
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_window("main") {
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                        width: 1280,
                        height: 960,
                    }));
                    let _ = window.set_resizable(true);
                }
            }

            // Initialize hardware controller
            let hw: HWController = HWController::new(app_handle.clone());
            app.manage(Mutex::new(hw.clone()));
            hw.init();

            // Initialize HEOS client
            let heos_client = HeosClient::new(heos_config);
            app.manage(Mutex::new(heos_client));

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running BS5 controller UI application");
}
