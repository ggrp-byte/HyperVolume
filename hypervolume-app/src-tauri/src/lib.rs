use tauri::{App, Manager};
use std::sync::Mutex;

mod audio_manager;
mod audio_boost;
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
            get_app_boost
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
