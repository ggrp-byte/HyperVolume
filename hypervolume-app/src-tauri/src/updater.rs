use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub changelog: String,
    pub mandatory: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl AppVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn from_string(version_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid version format".to_string());
        }

        let major = parts[0].parse::<u32>().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse::<u32>().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse::<u32>().map_err(|_| "Invalid patch version")?;

        Ok(Self::new(major, minor, patch))
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn is_newer_than(&self, other: &AppVersion) -> bool {
        if self.major > other.major {
            return true;
        }
        if self.major == other.major && self.minor > other.minor {
            return true;
        }
        if self.major == other.major && self.minor == other.minor && self.patch > other.patch {
            return true;
        }
        false
    }
}

pub struct UpdateManager {
    current_version: AppVersion,
    update_url: String,
}

impl UpdateManager {
    pub fn new(current_version: AppVersion) -> Self {
        Self {
            current_version,
            update_url: "https://api.github.com/repos/ggrp-byte/HyperVolume/releases/latest".to_string(),
        }
    }

    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, String> {
        let client = reqwest::Client::new();
        
        let response = client
            .get(&self.update_url)
            .header("User-Agent", "HyperVolume-Updater")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch update info: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let release_info: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let tag_name = release_info["tag_name"]
            .as_str()
            .ok_or("Missing tag_name in release info")?;

        // Remove 'v' prefix if present
        let version_str = tag_name.strip_prefix('v').unwrap_or(tag_name);
        let remote_version = AppVersion::from_string(version_str)?;

        if remote_version.is_newer_than(&self.current_version) {
            let download_url = release_info["assets"][0]["browser_download_url"]
                .as_str()
                .ok_or("No download URL found")?
                .to_string();

            let changelog = release_info["body"]
                .as_str()
                .unwrap_or("No changelog available")
                .to_string();

            Ok(Some(UpdateInfo {
                version: remote_version.to_string(),
                download_url,
                changelog,
                mandatory: false, // Could be determined by release notes or tags
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn download_update(&self, update_info: &UpdateInfo, download_path: &Path) -> Result<(), String> {
        let client = reqwest::Client::new();
        
        let response = client
            .get(&update_info.download_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download update: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read download bytes: {}", e))?;

        fs::write(download_path, bytes)
            .map_err(|e| format!("Failed to write update file: {}", e))?;

        Ok(())
    }

    pub fn install_update(&self, installer_path: &Path) -> Result<(), String> {
        // Launch the installer with silent installation flags
        let output = Command::new(installer_path)
            .arg("/S") // Silent installation
            .arg("/CLOSEAPPLICATIONS") // Close running applications
            .arg("/RESTARTAPPLICATIONS") // Restart applications after installation
            .output()
            .map_err(|e| format!("Failed to launch installer: {}", e))?;

        if !output.status.success() {
            return Err(format!("Installer failed with exit code: {:?}", output.status.code()));
        }

        Ok(())
    }

    pub fn schedule_restart(&self) -> Result<(), String> {
        // Schedule application restart after update
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to get current executable path: {}", e))?;

        // Create a batch script to restart the application
        let restart_script = format!(
            r#"@echo off
timeout /t 3 /nobreak > nul
start "" "{}"
del "%~f0""#,
            current_exe.display()
        );

        let script_path = std::env::temp_dir().join("hypervolume_restart.bat");
        fs::write(&script_path, restart_script)
            .map_err(|e| format!("Failed to create restart script: {}", e))?;

        // Launch the restart script
        Command::new("cmd")
            .args(&["/C", &script_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("Failed to launch restart script: {}", e))?;

        Ok(())
    }
}

// Auto-updater configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfig {
    pub auto_check: bool,
    pub check_interval_hours: u64,
    pub auto_download: bool,
    pub auto_install: bool,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            auto_check: true,
            check_interval_hours: 24, // Check daily
            auto_download: true,
            auto_install: false, // Require user confirmation for installation
        }
    }
}

pub fn load_update_config() -> UpdateConfig {
    let config_path = dirs::config_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("HyperVolume")
        .join("update_config.json");

    if let Ok(config_str) = fs::read_to_string(&config_path) {
        serde_json::from_str(&config_str).unwrap_or_default()
    } else {
        UpdateConfig::default()
    }
}

pub fn save_update_config(config: &UpdateConfig) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("HyperVolume");

    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_path = config_dir.join("update_config.json");
    let config_str = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, config_str)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

