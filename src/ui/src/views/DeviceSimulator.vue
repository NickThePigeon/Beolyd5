<script setup lang="ts">
import { ref, watch } from "vue";
import { useUIStore } from "../stores/ui.ts";
import FullscreenContainer from "./FullscreenContainer.vue";
import { Wheel } from "../hardware/events.ts";
import heosService, {
	type Player,
	type NowPlayingMedia,
} from "../services/heos.ts";

const uiStore = useUIStore();

// HEOS state
const heosHost = ref("192.168.1.2");
const heosConnected = ref(false);
const heosError = ref("");
const heosPlayer = ref<Player | null>(null);
const heosVolume = ref(0);
const heosMuted = ref(false);
const heosPlayState = ref("stop");
const heosNowPlaying = ref<NowPlayingMedia | null>(null);
const heosLoading = ref(false);

const handleAngularWheelChange = (e: Event) => {
	const val = Number((e.target as HTMLInputElement).value);
	uiStore.nextHardwareEvent({
		payload: { kind: "wheel", source: Wheel.Angular, value: val },
	});
};

// HEOS functions
async function heosConnect() {
	heosLoading.value = true;
	heosError.value = "";
	try {
		await heosService.setHost(heosHost.value);
		const player = await heosService.connectAndDiscover();
		heosPlayer.value = player;
		heosConnected.value = true;
		await refreshHeosState();
	} catch (e: any) {
		heosError.value = e.toString();
		heosConnected.value = false;
	} finally {
		heosLoading.value = false;
	}
}

async function heosDisconnect() {
	try {
		await heosService.disconnect();
	} catch (e) {
		// ignore
	}
	heosConnected.value = false;
	heosPlayer.value = null;
}

async function refreshHeosState() {
	if (!heosConnected.value) return;
	try {
		heosVolume.value = await heosService.getVolume();
		// Sync UI store volume with HEOS volume
		uiStore.volume = heosVolume.value;
		heosMuted.value = await heosService.getMute();
		heosPlayState.value = await heosService.getPlayState();
		heosNowPlaying.value = await heosService.getNowPlaying();
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

// Watch UI store volume changes and send to HEOS
let volumeDebounceTimer: number | null = null;
watch(
	() => uiStore.volume,
	(newVolume) => {
		if (!heosConnected.value) return;
		// Debounce volume changes to avoid spamming HEOS
		if (volumeDebounceTimer) {
			clearTimeout(volumeDebounceTimer);
		}
		volumeDebounceTimer = window.setTimeout(async () => {
			try {
				// Round to integer for HEOS API (expects u8)
				const volumeInt = Math.round(newVolume);
				await heosService.setVolume(volumeInt);
				heosVolume.value = volumeInt;
			} catch (e: any) {
				heosError.value = e.toString();
			}
		}, 100);
	},
);

async function heosSetVolume(level: number) {
	try {
		const volumeInt = Math.round(level);
		await heosService.setVolume(volumeInt);
		heosVolume.value = volumeInt;
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosVolumeUp() {
	try {
		await heosService.volumeUp(5);
		heosVolume.value = Math.min(100, heosVolume.value + 5);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosVolumeDown() {
	try {
		await heosService.volumeDown(5);
		heosVolume.value = Math.max(0, heosVolume.value - 5);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosToggleMute() {
	try {
		await heosService.toggleMute();
		heosMuted.value = !heosMuted.value;
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPlay() {
	try {
		await heosService.play();
		heosPlayState.value = "play";
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPause() {
	try {
		await heosService.pause();
		heosPlayState.value = "pause";
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosNext() {
	try {
		await heosService.playNext();
		setTimeout(refreshHeosState, 500);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPrevious() {
	try {
		await heosService.playPrevious();
		setTimeout(refreshHeosState, 500);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPlayTV() {
	try {
		await heosService.playTV();
		setTimeout(refreshHeosState, 500);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPlayLineIn() {
	try {
		await heosService.playInput("inputs/line_in_1");
		setTimeout(refreshHeosState, 500);
	} catch (e: any) {
		heosError.value = e.toString();
	}
}

async function heosPlaySpotify() {
	try {
		// Try to resume playback - works if Spotify was the last source
		await heosService.play();
		setTimeout(refreshHeosState, 500);
	} catch (e: any) {
		// If resume fails, show helpful message
		heosError.value =
			"Use Spotify app to select HEOS Woonkamer as playback device";
	}
}
</script>

<template>
	<a
		href="https://github.com/larsbaunwall/Beolyd5"
		class="github-corner"
		aria-label="View source on GitHub"
	>
		<svg
			width="80"
			height="80"
			viewBox="0 0 250 250"
			style="
				fill: #151513;
				color: #fff;
				position: absolute;
				top: 0;
				border: 0;
				right: 0;
			"
			aria-hidden="true"
		>
			<path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
			<path
				d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2"
				fill="currentColor"
				style="transform-origin: 130px 106px"
				class="octo-arm"
			></path>
			<path
				d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"
				fill="currentColor"
				class="octo-body"
			></path>
		</svg>
	</a>
	<div style="padding: 20px; background-color: white">
		<h1>Beolyd5 UI simulation</h1>
		<p>
			This is a simulation of the custom-built UI for the Beosound 5 sound
			system. The software is written for the Raspberry PI using the
			Beosound 5 rotation controller and a 1024x768px screen.
		</p>
		<p>
			Read more at
			<a href="https://github.com/larsbaunwall/Beolyd5"
				>the project repo on Github.</a
			>
		</p>
		<h2>How to use</h2>
		<p>
			Use the sliders to control the wheel pointer angle and the volume.
			The buttons are not functional yet.
		</p>
		<p>
			For now, you can use up and down arrow keys to cycle through the
			menu option list.
		</p>
		<h2>Contribute</h2>
		<p>
			Come help out - open an
			<a href="https://github.com/larsbaunwall/Beolyd5/issues">issue</a>
			or start a
			<a href="https://github.com/larsbaunwall/Beolyd5/discussions"
				>discussion</a
			>!
		</p>
		<div id="simulator">
			<div style="z-index: 10000">
				<span
					style="
						position: absolute;
						left: 1540px;
						top: 340px;
						color: gray;
					"
					>Wheels</span
				>
				<div id="wheel-bars">
					<div>
						<input
							type="range"
							min="150"
							max="210"
							step="0.1"
							disabled
						/>
					</div>
					<div>
						<input
							type="range"
							value="60"
							min="0"
							max="120"
							step="0.1"
							@input="handleAngularWheelChange"
						/>
					</div>
					<div>
						<input
							type="range"
							min="0"
							max="100"
							step="0.1"
							v-model="uiStore.volume"
						/>
					</div>
				</div>
				<img id="wheel" src="../assets/wheel.png" />
				<div class="controls">
					<button class="middle-button"><</button>
					<button class="middle-button">></button>
					<button class="middle-button">GO</button>
				</div>
			</div>
			<div class="debug-container">
				<FullscreenContainer />
			</div>
		</div>
		<div id="store-props">
			<div><strong>Debug values</strong></div>
			<div>
				<span>Wheel pointer angle: </span
				><input
					type="number"
					step="0.5"
					min="150"
					max="210"
					v-model="uiStore.wheelPointerAngle"
				/>
			</div>
			<div>
				<span>Volume: </span
				><input
					type="number"
					step="0.5"
					min="0"
					max="100"
					v-model="uiStore.volume"
				/>
			</div>
		</div>

		<!-- HEOS Control Panel -->
		<div id="heos-panel">
			<h2>HEOS Control Panel</h2>

			<!-- Connection -->
			<div class="heos-section">
				<h3>Connection</h3>
				<div class="heos-row">
					<label>Host IP:</label>
					<input
						type="text"
						v-model="heosHost"
						:disabled="heosConnected"
					/>
					<button
						v-if="!heosConnected"
						@click="heosConnect"
						:disabled="heosLoading"
					>
						{{ heosLoading ? "Connecting..." : "Connect" }}
					</button>
					<button v-else @click="heosDisconnect">Disconnect</button>
				</div>
				<div v-if="heosError" class="heos-error">{{ heosError }}</div>
				<div v-if="heosPlayer" class="heos-info">
					Connected to: <strong>{{ heosPlayer.name }}</strong> ({{
						heosPlayer.model
					}})
				</div>
			</div>

			<!-- Playback Controls -->
			<div v-if="heosConnected" class="heos-section">
				<h3>Playback</h3>
				<div class="heos-row">
					<button @click="heosPrevious">Prev</button>
					<button
						@click="heosPlay"
						:class="{ active: heosPlayState === 'play' }"
					>
						Play
					</button>
					<button
						@click="heosPause"
						:class="{ active: heosPlayState === 'pause' }"
					>
						Pause
					</button>
					<button @click="heosNext">Next</button>
					<button @click="refreshHeosState">Refresh</button>
				</div>
				<div class="heos-row">
					<span>State: {{ heosPlayState }}</span>
				</div>
			</div>

			<!-- Volume -->
			<div v-if="heosConnected" class="heos-section">
				<h3>
					Volume: {{ heosVolume }}% {{ heosMuted ? "(MUTED)" : "" }}
				</h3>
				<div class="heos-row">
					<button @click="heosVolumeDown">-</button>
					<input
						type="range"
						min="0"
						max="100"
						v-model.number="heosVolume"
						@change="heosSetVolume(heosVolume)"
						style="flex: 1"
					/>
					<button @click="heosVolumeUp">+</button>
					<button @click="heosToggleMute">
						{{ heosMuted ? "Unmute" : "Mute" }}
					</button>
				</div>
			</div>

			<!-- Now Playing -->
			<div v-if="heosConnected && heosNowPlaying" class="heos-section">
				<h3>Now Playing</h3>
				<div class="heos-now-playing">
					<img
						v-if="heosNowPlaying.image_url"
						:src="heosNowPlaying.image_url"
						alt="Album art"
					/>
					<div>
						<div v-if="heosNowPlaying.song">
							<strong>{{ heosNowPlaying.song }}</strong>
						</div>
						<div v-if="heosNowPlaying.artist">
							{{ heosNowPlaying.artist }}
						</div>
						<div v-if="heosNowPlaying.album">
							{{ heosNowPlaying.album }}
						</div>
						<div v-if="heosNowPlaying.station">
							Station: {{ heosNowPlaying.station }}
						</div>
					</div>
				</div>
			</div>

			<!-- Sources -->
			<div v-if="heosConnected" class="heos-section">
				<h3>Quick Sources</h3>
				<div class="heos-row">
					<button @click="heosPlayTV">TV (Optical)</button>
					<button @click="heosPlayLineIn">Line In</button>
					<button @click="heosPlaySpotify">Spotify</button>
				</div>
			</div>
		</div>
	</div>
</template>
<style scoped>
#wheel {
	position: absolute;
	top: 218px;
	left: 1004px;
	width: 500px;
	height: 500px;
	z-index: 1000;
}

.controls {
	display: flex;
	justify-content: space-between;
	position: absolute;
	top: 449px; /* Position at the middle of the parent element */
	left: 1057px; /* Position at the middle of the parent element */
	width: 390px; /* Adjust as needed */
	z-index: 2500;
}

.middle-button {
	flex: 1; /* This will make the buttons take up equal space */
	margin: 2px; /* Add some space between the buttons */
	height: 32px;
	background-color: black;
	color: white;
	border: none;
	cursor: pointer;
}

.middle-button:active {
	animation: flashGray 300ms;
}

@keyframes flashGray {
	0% {
		background-color: black;
	}
	50% {
		background-color: #333;
	}
	100% {
		background-color: black;
	}
}

.debug-container {
	border: 80px black solid;
	border-radius: 5px;
	width: 1024px; /* Adjust as needed */
	height: 768px; /* Adjust as needed */
	position: absolute; /* or relative, depending on your needs */
	overflow: hidden;
}

#simulator {
	position: relative;
	width: 1700px; /* Adjust as needed */
	height: 948px; /* Adjust as needed */
	overflow: auto;
	top: 20px; /* Adjust as needed */
	left: 20px; /* Adjust as needed */
}

#store-props {
	margin: 20px;
	display: flex;
	flex-direction: column;
}

#store-props div {
	margin: 2px;
}

#wheel-bars {
	display: flex;
	justify-content: space-between;
	flex-direction: row;
	position: absolute;
	top: 459px;
	left: 1520px;
	width: 100px;
	z-index: 111111;
}

#wheel-bars div {
	transform: rotate(270deg);
	width: 100px;
}

#wheel-bars input {
	width: 200px;
	margin: -90px;
}
.github-corner:hover .octo-arm {
	animation: octocat-wave 560ms ease-in-out;
}
@keyframes octocat-wave {
	0%,
	100% {
		transform: rotate(0);
	}
	20%,
	60% {
		transform: rotate(-25deg);
	}
	40%,
	80% {
		transform: rotate(10deg);
	}
}
@media (max-width: 500px) {
	.github-corner:hover .octo-arm {
		animation: none;
	}
	.github-corner .octo-arm {
		animation: octocat-wave 560ms ease-in-out;
	}
}

/* HEOS Control Panel Styles */
#heos-panel {
	margin: 20px;
	padding: 20px;
	background-color: #f5f5f5;
	border-radius: 8px;
	max-width: 600px;
}

#heos-panel h2 {
	margin-top: 0;
	margin-bottom: 15px;
	color: #333;
}

#heos-panel h3 {
	margin-top: 0;
	margin-bottom: 10px;
	color: #555;
	font-size: 14px;
}

.heos-section {
	margin-bottom: 20px;
	padding: 15px;
	background-color: white;
	border-radius: 6px;
	border: 1px solid #ddd;
}

.heos-row {
	display: flex;
	align-items: center;
	gap: 10px;
	margin-bottom: 8px;
}

.heos-row:last-child {
	margin-bottom: 0;
}

.heos-row label {
	min-width: 60px;
	font-weight: 500;
}

.heos-row input[type="text"] {
	flex: 1;
	padding: 6px 10px;
	border: 1px solid #ccc;
	border-radius: 4px;
}

.heos-row button {
	padding: 6px 12px;
	background-color: #333;
	color: white;
	border: none;
	border-radius: 4px;
	cursor: pointer;
	transition: background-color 0.2s;
}

.heos-row button:hover {
	background-color: #555;
}

.heos-row button:disabled {
	background-color: #999;
	cursor: not-allowed;
}

.heos-row button.active {
	background-color: #007bff;
}

.heos-error {
	margin-top: 10px;
	padding: 8px 12px;
	background-color: #fee;
	color: #c00;
	border-radius: 4px;
	font-size: 13px;
}

.heos-info {
	margin-top: 10px;
	padding: 8px 12px;
	background-color: #efe;
	color: #060;
	border-radius: 4px;
	font-size: 13px;
}

.heos-now-playing {
	display: flex;
	gap: 15px;
	align-items: flex-start;
}

.heos-now-playing img {
	width: 80px;
	height: 80px;
	object-fit: cover;
	border-radius: 4px;
}

.heos-now-playing div {
	flex: 1;
}

.heos-now-playing div div {
	margin-bottom: 4px;
}
</style>
