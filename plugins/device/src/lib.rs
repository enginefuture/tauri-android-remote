#[cfg(target_os = "android")]
use tauri::Manager;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("device")
        .setup(|_app, _api| {
            #[cfg(target_os = "android")]
            {
                let handle = _api.register_android_plugin(
                    "io.enginefuture.androidremote.device",
                    "DevicePlugin",
                )?;
                _app.manage(handle);
            }
            Ok(())
        })
        .build()
}
pub fn call<R: Runtime>(
    _app: &tauri::AppHandle<R>,
    _method: &str,
) -> Result<serde_json::Value, String> {
    #[cfg(target_os = "android")]
    {
        _app.state::<tauri::plugin::PluginHandle<R>>()
            .run_mobile_plugin(_method, ())
            .map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "android"))]
    {
        Err("此功能需要在安卓手机中运行".into())
    }
}
