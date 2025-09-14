import { invoke } from "@tauri-apps/api/core";

interface AudioSession {
  id: string;
  display_name: string;
  volume: number;
  muted: boolean;
  process_id: number;
}

interface UpdateInfo {
  version: string;
  download_url: string;
  changelog: string;
  mandatory: boolean;
}

interface UpdateConfig {
  auto_check: boolean;
  check_interval_hours: number;
  auto_download: boolean;
  auto_install: boolean;
}

let audioSessions: AudioSession[] = [];
let updateCheckInterval: number | null = null;

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

async function checkForUpdates() {
  try {
    const updateInfo: UpdateInfo | null = await invoke("check_for_updates");
    if (updateInfo) {
      showUpdateNotification(updateInfo);
    }
  } catch (error) {
    console.error("Failed to check for updates:", error);
  }
}

async function downloadAndInstallUpdate(updateInfo: UpdateInfo) {
  try {
    showUpdateProgress("Downloading update...");
    await invoke("download_and_install_update", { updateInfo });
    showUpdateProgress("Update installed successfully! Restarting...");
  } catch (error) {
    console.error("Failed to install update:", error);
    showUpdateProgress("Update failed: " + error);
  }
}

function showUpdateNotification(updateInfo: UpdateInfo) {
  const notification = document.createElement("div");
  notification.className = "update-notification";
  notification.innerHTML = `
    <div class="update-header">
      <h3>🎉 Update Available: v${updateInfo.version}</h3>
      <button class="close-btn" onclick="this.parentElement.parentElement.remove()">×</button>
    </div>
    <div class="update-content">
      <p><strong>Changelog:</strong></p>
      <div class="changelog">${updateInfo.changelog}</div>
      <div class="update-actions">
        <button class="update-btn" onclick="installUpdate('${JSON.stringify(updateInfo).replace(/'/g, "\\'")}')">
          Install Update
        </button>
        <button class="later-btn" onclick="this.parentElement.parentElement.parentElement.remove()">
          Later
        </button>
      </div>
    </div>
  `;
  
  document.body.appendChild(notification);
}

function showUpdateProgress(message: string) {
  let progressDiv = document.querySelector(".update-progress") as HTMLElement;
  if (!progressDiv) {
    progressDiv = document.createElement("div");
    progressDiv.className = "update-progress";
    document.body.appendChild(progressDiv);
  }
  progressDiv.textContent = message;
}

// Global function for onclick handlers
(window as any).installUpdate = async (updateInfoJson: string) => {
  const updateInfo = JSON.parse(updateInfoJson);
  await downloadAndInstallUpdate(updateInfo);
};

async function setupAutoUpdater() {
  try {
    const config: UpdateConfig = await invoke("get_update_config");
    
    if (config.auto_check) {
      // Check for updates on startup
      setTimeout(checkForUpdates, 5000); // Wait 5 seconds after startup
      
      // Set up periodic checks
      if (updateCheckInterval) {
        clearInterval(updateCheckInterval);
      }
      
      updateCheckInterval = setInterval(
        checkForUpdates,
        config.check_interval_hours * 60 * 60 * 1000
      );
    }
  } catch (error) {
    console.error("Failed to setup auto-updater:", error);
  }
}

function renderAudioSessions() {
  const container = document.querySelector("#sessions-container");
  if (!container) return;

  container.innerHTML = "";

  if (audioSessions.length === 0) {
    container.innerHTML = `
      <div class="no-sessions">
        <p>No active audio sessions found.</p>
        <p>Start playing audio in any application to see it here.</p>
      </div>
    `;
    return;
  }

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
  setupAutoUpdater();
  
  // Refresh sessions every 2 seconds
  setInterval(loadAudioSessions, 2000);
  
  // Add refresh button listener
  document.querySelector("#refresh-btn")?.addEventListener("click", loadAudioSessions);
  
  // Add manual update check button listener
  document.querySelector("#check-updates-btn")?.addEventListener("click", checkForUpdates);
});

