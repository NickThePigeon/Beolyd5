<template>
	<div class="container">
		<MainCircleArc :radius="radius" />
		<VolumeArc />
		<AnglePointer :radius="radius" />

		<!-- Now Playing Box in Center -->
		<div class="now-playing-box">
			<div v-if="isConnected && nowPlaying" class="now-playing-content">
				<img
					v-if="nowPlaying.image_url"
					:src="nowPlaying.image_url"
					class="album-art"
					alt="Album art"
				/>
				<div class="track-info">
					<div class="track-title">
						{{ nowPlaying.song || nowPlaying.station || "Unknown" }}
					</div>
					<div class="track-artist">
						{{ nowPlaying.artist || "" }}
					</div>
					<div class="track-album">{{ nowPlaying.album || "" }}</div>
				</div>
			</div>
			<div v-else-if="isConnected" class="now-playing-empty">
				<div class="no-media">No media playing</div>
			</div>
			<div v-else class="now-playing-disconnected">
				<div class="not-connected">Not connected</div>
			</div>
		</div>

		<!-- Menu Items -->
		<div
			v-for="(item, index) in menuItems"
			:key="index"
			class="list-item"
			:style="menuItemStyle(index)"
			:class="{
				selectedItem: isSelectedItem(index),
				activeSource: item.source === currentSource,
				standbyActive: item.action === 'standby' && isStandby,
			}"
		>
			{{
				item.action === "standby"
					? isStandby
						? "STANDBY"
						: "ON"
					: item.title
			}}
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, CSSProperties, ref, onMounted, onUnmounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useUIStore } from "../stores/ui.ts";
import arcs from "../utils/arcs.ts";
import MainCircleArc from "../components/MainArc.vue";
import VolumeArc from "../components/VolumeArc.vue";
import AnglePointer from "../components/AnglePointer.vue";
import heosService, { type NowPlayingMedia } from "../services/heos.ts";

const router = useRouter();
const uiStore = useUIStore();

// Now Playing state
const nowPlaying = ref<NowPlayingMedia | null>(null);
const currentSource = ref<string>("");
const isConnected = ref(false);

// Poll now playing info
let pollInterval: number | null = null;

async function refreshNowPlaying() {
	try {
		// First check if we're connected
		const connected = await heosService.isConnected();
		isConnected.value = connected;

		if (connected) {
			nowPlaying.value = await heosService.getNowPlaying();
		} else {
			nowPlaying.value = null;
		}
	} catch (e) {
		// Not connected or error
		isConnected.value = false;
		nowPlaying.value = null;
	}
}

onMounted(() => {
	// Only start polling, don't immediately call (avoid error on startup)
	pollInterval = window.setInterval(refreshNowPlaying, 5000);
});

onUnmounted(() => {
	if (pollInterval) {
		clearInterval(pollInterval);
	}
});

// Toggle standby mode
async function toggleStandby() {
	try {
		if (isStandby.value) {
			// Wake up - resume playback
			await heosService.play();
			isStandby.value = false;
		} else {
			// Go to standby - stop playback
			await heosService.stop();
			isStandby.value = true;
		}
		setTimeout(refreshNowPlaying, 500);
	} catch (e) {
		console.error("Failed to toggle standby:", e);
	}
}

// Select source or perform action when Go button is pressed
async function selectSource(source: string | null, action: string | null) {
	// Handle actions first
	if (action === "standby") {
		await toggleStandby();
		return;
	}

	if (!source) return;

	try {
		if (source === "spotify") {
			// Resume Spotify playback
			await heosService.play();
		} else {
			// Switch to input
			await heosService.playInput(source);
		}
		currentSource.value = source;
		isStandby.value = false; // No longer in standby when selecting a source
		// Refresh now playing after a short delay
		setTimeout(refreshNowPlaying, 500);
	} catch (e) {
		console.error("Failed to select source:", e);
	}
}

const route = useRoute();

const menuItems = ref([
	{ title: "STANDBY", path: "menu", source: null, action: "standby" },
	{ title: "N.SPOTIFY", path: "menu", source: "spotify", action: null },
	{
		title: "N.TV",
		path: "menu",
		source: "inputs/optical_in_1",
		action: null,
	},
	{
		title: "N.PHONE",
		path: "menu",
		source: "inputs/line_in_1",
		action: null,
	},
]);

// Standby state
const isStandby = ref(false);
const radius = ref(1000);
const angleStep = ref(7);

const startItemAngle = computed(() => {
	const totalSpan = angleStep.value * (menuItems.value.length - 1);
	return 180 - totalSpan / 2;
});

function menuItemStyle(index: number): CSSProperties {
	const itemAngle = startItemAngle.value + index * angleStep.value;
	const position = arcs.getArcPoint(radius.value, 20, itemAngle);
	const divWidth = 100;
	const divHeight = 50;
	return {
		position: "absolute",
		left: `${position.x - divWidth}px`,
		top: `${position.y - divHeight / 2}px`,
		width: `${divWidth}px`,
		height: `${divHeight}px`,
	};
}

const selectedMenuItem = ref(-1);

// Handle Go button press - select current source or action
function handleGoButton() {
	if (selectedMenuItem.value >= 0) {
		const item = menuItems.value[selectedMenuItem.value];
		selectSource(item.source, item.action);
	}
}

// Listen for keyboard events (Go button simulation)
function handleKeydown(e: KeyboardEvent) {
	if (e.key === "Enter" || e.key === " ") {
		handleGoButton();
	}
}

onMounted(() => {
	window.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
	window.removeEventListener("keydown", handleKeydown);
});

function isSelectedItem(index: number) {
	const itemAngle = startItemAngle.value + index * angleStep.value;
	const diff = Math.abs(uiStore.wheelPointerAngle - itemAngle);
	if (diff <= 2) {
		if (selectedMenuItem.value !== index) {
			uiStore.tick();
			selectedMenuItem.value = index;
			router.push({
				path: `/${route.params.shell ?? "default"}/${menuItems.value[index].path}`,
			});
		}
		return true;
	}
	return false;
}
</script>

<style scoped>
.container {
	background-color: black;
	width: 100%;
	height: 1000px;
	cursor: none;
	overflow: hidden;
}

.list-item {
	z-index: 1000;
	font-weight: 100;
	font-size: 14px;
	color: white;
	display: flex;
	justify-content: right;
	align-items: center;
	transition: font-weight 0.5s ease-in-out;
}

.list-item.selectedItem {
	font-weight: bold;
	transition: font-weight 0.5s ease-in-out;
}

.list-item.activeSource {
	color: #4caf50;
}

.list-item.standbyActive {
	color: #ff5722;
}

/* Now Playing Box */
.now-playing-box {
	position: absolute;
	top: 50%;
	left: 50%;
	transform: translate(-50%, -50%);
	width: 400px;
	height: 200px;
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 100;
}

.now-playing-content {
	display: flex;
	align-items: center;
	gap: 20px;
	width: 100%;
}

.album-art {
	width: 150px;
	height: 150px;
	object-fit: cover;
	border-radius: 4px;
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

.track-info {
	flex: 1;
	color: white;
	overflow: hidden;
}

.track-title {
	font-size: 24px;
	font-weight: bold;
	margin-bottom: 8px;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}

.track-artist {
	font-size: 18px;
	color: #aaa;
	margin-bottom: 4px;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}

.track-album {
	font-size: 14px;
	color: #777;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}

.now-playing-empty,
.now-playing-disconnected {
	display: flex;
	align-items: center;
	justify-content: center;
	width: 100%;
	height: 100%;
}

.no-media,
.not-connected {
	color: #555;
	font-size: 18px;
	font-style: italic;
}
</style>
