mod timer;

use timer::{Phase, SharedTimerService, TimerState, create_timer_service};

#[tauri::command]
fn get_state(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    Ok(service.get_state())
}

#[tauri::command]
fn start_timer(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.start()
}

#[tauri::command]
fn pause_timer(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.pause()
}

#[tauri::command]
fn resume_timer(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.resume()
}

#[tauri::command]
fn clear_timer(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.clear()
}

#[tauri::command]
fn set_phase(phase: String, timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let phase_enum = match phase.to_lowercase().as_str() {
        "work" => Phase::Work,
        "break" => Phase::Break,
        _ => return Err("Invalid phase. Use 'work' or 'break'.".to_string()),
    };

    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.set_phase(phase_enum);
    Ok(service.get_state())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(create_timer_service())
        .invoke_handler(tauri::generate_handler![
            get_state,
            start_timer,
            pause_timer,
            resume_timer,
            clear_timer,
            set_phase
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
