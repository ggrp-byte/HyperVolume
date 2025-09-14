use windows::core::HSTRING;
use windows::Win32::Foundation::PWSTR;
use windows::Win32::Media::Audio::CoreAudio::{
    eRender,
    IMMDeviceEnumerator,
    MMDeviceEnumerator,
    DEVICE_STATE_ACTIVE,
    IAudioSessionManager2,
    ISimpleAudioVolume,
    IAudioSessionControl2,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioSession {
    pub id: String,
    pub display_name: String,
    pub volume: f32,
    pub muted: bool,
    pub process_id: u32,
}

pub fn enumerate_audio_sessions() -> Result<Vec<AudioSession>, String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).map_err(|e| format!("Failed to initialize COM: {:?}", e))?;

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create device enumerator: {:?}", e))?;

        let device = enumerator.GetDefaultAudioEndpoint(eRender, DEVICE_STATE_ACTIVE)
            .map_err(|e| format!("Failed to get default audio endpoint: {:?}", e))?;

        let session_manager2: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)
            .map_err(|e| format!("Failed to activate audio session manager: {:?}", e))?;

        let session_enumerator = session_manager2.GetSessionEnumerator()
            .map_err(|e| format!("Failed to get session enumerator: {:?}", e))?;

        let count = session_enumerator.GetCount()
            .map_err(|e| format!("Failed to get session count: {:?}", e))?;

        let mut sessions = Vec::new();

        for i in 0..count {
            let session_control = session_enumerator.GetSession(i)
                .map_err(|e| format!("Failed to get session control: {:?}", e))?;

            let session_control2: IAudioSessionControl2 = session_control.cast()
                .map_err(|e| format!("Failed to cast to IAudioSessionControl2: {:?}", e))?;

            let process_id = session_control2.GetProcessId()
                .map_err(|e| format!("Failed to get process ID: {:?}", e))?;

            let display_name_pwstr = session_control2.GetDisplayName()
                .map_err(|e| format!("Failed to get display name: {:?}", e))?;
            let display_name = pwstr_to_string(display_name_pwstr);

            let simple_audio_volume: ISimpleAudioVolume = session_control.cast()
                .map_err(|e| format!("Failed to cast to ISimpleAudioVolume: {:?}", e))?;

            let mut volume = 0.0;
            simple_audio_volume.GetMasterVolume(&mut volume)
                .map_err(|e| format!("Failed to get master volume: {:?}", e))?;

            let mut muted = 0;
            simple_audio_volume.GetMute(&mut muted)
                .map_err(|e| format!("Failed to get mute state: {:?}", e))?;

            sessions.push(AudioSession {
                id: format!("{}", process_id), // Use process ID as a simple ID for now
                display_name,
                volume,
                muted: muted != 0,
                process_id,
            });
        }
        Ok(sessions)
    }
}

fn pwstr_to_string(pwstr: PWSTR) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *pwstr.0.add(i) != 0).count();
        let slice = std::slice::from_raw_parts(pwstr.0, len);
        String::from_utf16_lossy(slice)
    }
}

pub fn set_session_volume(process_id: u32, volume: f32) -> Result<(), String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).map_err(|e| format!("Failed to initialize COM: {:?}", e))?;

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create device enumerator: {:?}", e))?;

        let device = enumerator.GetDefaultAudioEndpoint(eRender, DEVICE_STATE_ACTIVE)
            .map_err(|e| format!("Failed to get default audio endpoint: {:?}", e))?;

        let session_manager2: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)
            .map_err(|e| format!("Failed to activate audio session manager: {:?}", e))?;

        let session_enumerator = session_manager2.GetSessionEnumerator()
            .map_err(|e| format!("Failed to get session enumerator: {:?}", e))?;

        let count = session_enumerator.GetCount()
            .map_err(|e| format!("Failed to get session count: {:?}", e))?;

        for i in 0..count {
            let session_control = session_enumerator.GetSession(i)
                .map_err(|e| format!("Failed to get session control: {:?}", e))?;

            let session_control2: IAudioSessionControl2 = session_control.cast()
                .map_err(|e| format!("Failed to cast to IAudioSessionControl2: {:?}", e))?;

            let current_process_id = session_control2.GetProcessId()
                .map_err(|e| format!("Failed to get process ID: {:?}", e))?;

            if current_process_id == process_id {
                let simple_audio_volume: ISimpleAudioVolume = session_control.cast()
                    .map_err(|e| format!("Failed to cast to ISimpleAudioVolume: {:?}", e))?;

                simple_audio_volume.SetMasterVolume(volume, None)
                    .map_err(|e| format!("Failed to set master volume: {:?}", e))?;
                return Ok(());
            }
        }
        Err(format!("Session with process ID {} not found", process_id))
    }
}

pub fn toggle_session_mute(process_id: u32) -> Result<(), String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).map_err(|e| format!("Failed to initialize COM: {:?}", e))?;

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create device enumerator: {:?}", e))?;

        let device = enumerator.GetDefaultAudioEndpoint(eRender, DEVICE_STATE_ACTIVE)
            .map_err(|e| format!("Failed to get default audio endpoint: {:?}", e))?;

        let session_manager2: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)
            .map_err(|e| format!("Failed to activate audio session manager: {:?}", e))?;

        let session_enumerator = session_manager2.GetSessionEnumerator()
            .map_err(|e| format!("Failed to get session enumerator: {:?}", e))?;

        let count = session_enumerator.GetCount()
            .map_err(|e| format!("Failed to get session count: {:?}", e))?;

        for i in 0..count {
            let session_control = session_enumerator.GetSession(i)
                .map_err(|e| format!("Failed to get session control: {:?}", e))?;

            let session_control2: IAudioSessionControl2 = session_control.cast()
                .map_err(|e| format!("Failed to cast to IAudioSessionControl2: {:?}", e))?;

            let current_process_id = session_control2.GetProcessId()
                .map_err(|e| format!("Failed to get process ID: {:?}", e))?;

            if current_process_id == process_id {
                let simple_audio_volume: ISimpleAudioVolume = session_control.cast()
                    .map_err(|e| format!("Failed to cast to ISimpleAudioVolume: {:?}", e))?;

                let mut muted = 0;
                simple_audio_volume.GetMute(&mut muted)
                    .map_err(|e| format!("Failed to get mute state: {:?}", e))?;

                simple_audio_volume.SetMute(muted == 0, None)
                    .map_err(|e| format!("Failed to set mute state: {:?}", e))?;
                return Ok(());
            }
        }
        Err(format!("Session with process ID {} not found", process_id))
    }
}

