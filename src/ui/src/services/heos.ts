/**
 * HEOS Service - TypeScript wrapper for Tauri HEOS commands
 */

import { invoke } from "@tauri-apps/api/tauri";

// ============================================================================
// Types
// ============================================================================

export interface Player {
	name: string;
	pid: number;
	model: string;
	version: string;
	ip?: string;
	network?: string;
	lineout?: number;
	serial?: string;
}

export interface NowPlayingMedia {
	type?: string;
	song?: string;
	album?: string;
	artist?: string;
	image_url?: string;
	mid?: string;
	qid?: number;
	sid?: number;
	station?: string;
}

export interface MusicSource {
	name: string;
	image_url?: string;
	type: string;
	sid: number;
	available?: string;
	service_username?: string;
}

export type PlayState = "play" | "pause" | "stop";

// ============================================================================
// Connection
// ============================================================================

/**
 * Set the HEOS device host IP address
 */
export async function setHost(host: string): Promise<void> {
	return invoke("heos_set_host", { host });
}

/**
 * Get the current HEOS host IP
 */
export async function getHost(): Promise<string> {
	return invoke("heos_get_host");
}

/**
 * Connect to HEOS device
 */
export async function connect(): Promise<void> {
	return invoke("heos_connect");
}

/**
 * Disconnect from HEOS device
 */
export async function disconnect(): Promise<void> {
	return invoke("heos_disconnect");
}

/**
 * Send heartbeat to check connection
 */
export async function heartbeat(): Promise<void> {
	return invoke("heos_heartbeat");
}

/**
 * Check if connected to HEOS (via heartbeat)
 */
export async function isConnected(): Promise<boolean> {
	try {
		await heartbeat();
		return true;
	} catch {
		return false;
	}
}

/**
 * Connect and auto-discover players, optionally selecting by name
 */
export async function connectAndDiscover(playerName?: string): Promise<Player> {
	return invoke("heos_connect_and_discover", { playerName });
}

// ============================================================================
// Player Discovery
// ============================================================================

/**
 * Get all HEOS players on the network
 */
export async function getPlayers(): Promise<Player[]> {
	return invoke("heos_get_players");
}

/**
 * Set the active player by ID
 */
export async function setPlayerId(pid: number): Promise<void> {
	return invoke("heos_set_player_id", { pid });
}

// ============================================================================
// Volume Control
// ============================================================================

/**
 * Get current volume (0-100)
 */
export async function getVolume(): Promise<number> {
	return invoke("heos_get_volume");
}

/**
 * Set volume (0-100)
 */
export async function setVolume(level: number): Promise<void> {
	return invoke("heos_set_volume", { level });
}

/**
 * Increase volume by step (1-10)
 */
export async function volumeUp(step: number = 5): Promise<void> {
	return invoke("heos_volume_up", { step });
}

/**
 * Decrease volume by step (1-10)
 */
export async function volumeDown(step: number = 5): Promise<void> {
	return invoke("heos_volume_down", { step });
}

// ============================================================================
// Mute Control
// ============================================================================

/**
 * Get mute state
 */
export async function getMute(): Promise<boolean> {
	return invoke("heos_get_mute");
}

/**
 * Set mute state
 */
export async function setMute(muted: boolean): Promise<void> {
	return invoke("heos_set_mute", { muted });
}

/**
 * Toggle mute
 */
export async function toggleMute(): Promise<void> {
	return invoke("heos_toggle_mute");
}

// ============================================================================
// Playback Control
// ============================================================================

/**
 * Get current play state
 */
export async function getPlayState(): Promise<PlayState> {
	return invoke("heos_get_play_state");
}

/**
 * Start playback
 */
export async function play(): Promise<void> {
	return invoke("heos_play");
}

/**
 * Pause playback
 */
export async function pause(): Promise<void> {
	return invoke("heos_pause");
}

/**
 * Stop playback
 */
export async function stop(): Promise<void> {
	return invoke("heos_stop");
}

/**
 * Toggle play/pause
 */
export async function togglePlayPause(): Promise<PlayState> {
	return invoke("heos_toggle_play_pause");
}

/**
 * Play next track
 */
export async function playNext(): Promise<void> {
	return invoke("heos_play_next");
}

/**
 * Play previous track
 */
export async function playPrevious(): Promise<void> {
	return invoke("heos_play_previous");
}

// ============================================================================
// Now Playing
// ============================================================================

/**
 * Get now playing media info
 */
export async function getNowPlaying(): Promise<NowPlayingMedia> {
	return invoke("heos_get_now_playing");
}

// ============================================================================
// Sources
// ============================================================================

/**
 * Get available music sources
 */
export async function getMusicSources(): Promise<MusicSource[]> {
	return invoke("heos_get_music_sources");
}

/**
 * Play an input source (e.g., "inputs/optical_in_1")
 */
export async function playInput(input: string): Promise<void> {
	return invoke("heos_play_input", { input });
}

/**
 * Play TV input (optical_in_1)
 */
export async function playTV(): Promise<void> {
	return invoke("heos_play_tv");
}

/**
 * Play a preset from HEOS Favorites
 */
export async function playPreset(preset: number): Promise<void> {
	return invoke("heos_play_preset", { preset });
}

/**
 * Clear the playback queue
 */
export async function clearQueue(): Promise<void> {
	return invoke("heos_clear_queue");
}

// ============================================================================
// Input Constants
// ============================================================================

export const Inputs = {
	AUX_IN_1: "inputs/aux_in_1",
	AUX_IN_2: "inputs/aux_in_2",
	OPTICAL_IN_1: "inputs/optical_in_1",
	OPTICAL_IN_2: "inputs/optical_in_2",
	COAX_IN_1: "inputs/coax_in_1",
	COAX_IN_2: "inputs/coax_in_2",
	HDMI_IN_1: "inputs/hdmi_in_1",
	HDMI_IN_2: "inputs/hdmi_in_2",
	HDMI_ARC_1: "inputs/hdmi_arc_1",
	TV_AUDIO: "inputs/tvaudio",
} as const;

// ============================================================================
// Convenience: Full HEOS service object
// ============================================================================

export const heosService = {
	// Connection
	setHost,
	getHost,
	connect,
	disconnect,
	heartbeat,
	isConnected,
	connectAndDiscover,

	// Players
	getPlayers,
	setPlayerId,

	// Volume
	getVolume,
	setVolume,
	volumeUp,
	volumeDown,

	// Mute
	getMute,
	setMute,
	toggleMute,

	// Playback
	getPlayState,
	play,
	pause,
	stop,
	togglePlayPause,
	playNext,
	playPrevious,

	// Now Playing
	getNowPlaying,

	// Sources
	getMusicSources,
	playInput,
	playTV,
	playPreset,
	clearQueue,

	// Constants
	Inputs,
};

export default heosService;
