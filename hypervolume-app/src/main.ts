import { invoke } from "@tauri-apps/api/core";

interface AudioSession {
  id: string;
  display_name: string;
  volume: number;
  muted: boolean;
  process_id: number;
}

let audioSessions: AudioSession[] = [];

async function loadAudioSessions() {
  try {
    audioSessions = await invoke("get_audio_sessions");
    renderAudioSessions();
  } catch (error) {
    console.error("Failed to load audio sessions:", error);
  }
}

async function setVolume(processId: number, volume: number) {
  try {
    await invoke("set_app_volume", { processId, volume });
    await loadAudioSessions(); // Refresh the list
  } catch (error) {
    console.error("Failed to set volume:", error);
  }
}

async function toggleMute(processId: number) {
  try {
    await invoke("toggle_app_mute", { processId });
    await loadAudioSessions(); // Refresh the list
  } catch (error) {
    console.error("Failed to toggle mute:", error);
  }
}

function renderAudioSessions() {
  const container = document.querySelector("#sessions-container");
  if (!container) return;

  container.innerHTML = "";

  audioSessions.forEach((session) => {
    const sessionElement = document.createElement("div");
    sessionElement.className = "session-item";
    
    const volumePercent = Math.round(session.volume * 100);
    const extendedVolumePercent = Math.round(session.volume * 777);
    
    sessionElement.innerHTML = `
      <div class="session-header">
        <div class="session-info">
          <span class="session-name">${session.display_name || `Process ${session.process_id}`}</span>
          <span class="session-volume">${extendedVolumePercent}%</span>
        </div>
        <button class="mute-btn ${session.muted ? 'muted' : ''}" data-process-id="${session.process_id}">
          ${session.muted ? '🔇' : '🔊'}
        </button>
      </div>
      <div class="volume-control">
        <input 
          type="range" 
          class="volume-slider" 
          min="0" 
          max="7.77" 
          step="0.01" 
          value="${session.volume}" 
          data-process-id="${session.process_id}"
        />
        <div class="volume-markers">
          <span class="marker" style="left: 12.87%">100%</span>
          <span class="marker" style="left: 100%">777%</span>
        </div>
      </div>
    `;

    container.appendChild(sessionElement);
  });

  // Add event listeners
  document.querySelectorAll(".volume-slider").forEach((slider) => {
    slider.addEventListener("input", (e) => {
      const target = e.target as HTMLInputElement;
      const processId = parseInt(target.dataset.processId!);
      const volume = parseFloat(target.value);
      setVolume(processId, volume);
    });
  });

  document.querySelectorAll(".mute-btn").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const target = e.target as HTMLButtonElement;
      const processId = parseInt(target.dataset.processId!);
      toggleMute(processId);
    });
  });
}

window.addEventListener("DOMContentLoaded", () => {
  loadAudioSessions();
  
  // Refresh sessions every 2 seconds
  setInterval(loadAudioSessions, 2000);
  
  // Add refresh button listener
  document.querySelector("#refresh-btn")?.addEventListener("click", loadAudioSessions);
});

