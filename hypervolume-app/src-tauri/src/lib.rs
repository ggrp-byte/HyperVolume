use tauri::{App, Manager};
use std::sync::Mutex;

mod audio_manager;
mod audio_boost;
mod updater;
mod tests;

// Global boost manager instance
static BOOST_MANAGER: Mutex<Option<audio_boost::AudioBoostManager>> = Mutex::new(None);

fn get_boost_manager() -> &'static Mutex<Option<audio_boost::AudioBoostManager>> {
    &BOOST_MANAGER
}

#[tauri::command]
fn get_audio_sessions() -> Result<Vec<audio_manager::AudioSession>, String> {
    audio_manager::enumerate_audio_sessions()
}

#[tauri::command]
fn set_app_volume(process_id: u32, volume: f32) -> Result<(), String> {
    // If volume > 1.0, use boost manager
    if volume > 1.0 {
        let manager_guard = get_boost_manager().lock().map_err(|e| format!("Lock error: {:?}", e))?;
        if let Some(ref manager) = *manager_guard {
            manager.set_boost(process_id, volume)
        } else {
            Err("Boost manager not initialized".to_string())
        }
    } else {
        // Use standard volume control
        audio_manager::set_session_volume(process_id, volume)
    }
}

#[tauri::command]
fn toggle_app_mute(process_id: u32) -> Result<(), String> {
    audio_manager::toggle_session_mute(process_id)
}

#[tauri::command]
fn set_app_boost(process_id: u32, boost_factor: f32) -> Result<(), String> {
    let manager_guard = get_boost_manager().lock().map_err(|e| format!("Lock error: {:?}", e))?;
    if let Some(ref manager) = *manager_guard {
        manager.set_boost(process_id, boost_factor)
    } else {
        Err("Boost manager not initialized".to_string())
    }
}

#[tauri::command]
fn get_app_boost(process_id: u32) -> Result<f32, String> {
    let manager_guard = get_boost_manager().lock().map_err(|e| format!("Lock error: {:?}", e))?;
    if let Some(ref manager) = *manager_guard {
        manager.get_boost(process_id)
    } else {
        Ok(1.0) // Default boost
    }
}

#[tauri::command]
async fn check_for_updates() -> Result<Option<updater::UpdateInfo>, String> {
    let current_version = updater::AppVersion::new(1, 0, 0); // Current app version
    let update_manager = updater::UpdateManager::new(current_version);
    update_manager.check_for_updates().await
}

#[tauri::command]
async fn download_and_install_update(update_info: updater::UpdateInfo) -> Result<(), String> {
    let current_version = updater::AppVersion::new(1, 0, 0);
    let update_manager = updater::UpdateManager::new(current_version);
    
    let download_path = std::env::temp_dir().join("HyperVolume-Update.exe");
    
    // Download the update
    update_manager.download_update(&update_info, &download_path).await?;
    
    // Install the update
    update_manager.install_update(&download_path)?;
    
    // Schedule restart
    update_manager.schedule_restart()?;
    
    Ok(())
}

#[tauri::command]
fn get_update_config() -> updater::UpdateConfig {
    updater::load_update_config()
}

#[tauri::command]
fn save_update_config(config: updater::UpdateConfig) -> Result<(), String> {
    updater::save_update_config(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize boost manager
    {
        let mut manager_guard = get_boost_manager().lock().unwrap();
        *manager_guard = Some(audio_boost::AudioBoostManager::new());
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_audio_sessions,
            set_app_volume,
            toggle_app_mute,
            set_app_boost,
            get_app_boost,
            check_for_updates,
            download_and_install_update,
            get_update_config,
            save_update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
