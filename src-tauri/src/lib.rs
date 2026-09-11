#[tauri::command]
fn connection_commands(ip: String, port: u16) -> Result<String, String> {
    remote_core::Connection { ip, port }.commands()
}
#[tauri::command]
fn device_status(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    tauri_plugin_device::call(&app, "status")
}
#[tauri::command]
fn open_developer_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    tauri_plugin_device::call(&app, "openSettings")
}
#[tauri::command]
fn open_tailscale(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    tauri_plugin_device::call(&app, "openTailscale")
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_device::init())
        .invoke_handler(tauri::generate_handler![
            connection_commands,
            device_status,
            open_developer_settings,
            open_tailscale
        ])
        .run(tauri::generate_context!())
        .expect("Tauri runtime failed");
}
