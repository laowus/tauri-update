#[tauri::command]
async fn get_app_info(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let version = app_handle.package_info().version.to_string();
    let name = app_handle.package_info().name.to_string();

    Ok(serde_json::json!({
        "version": version,
        "name": name
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(tauri_plugin_http::init());
    
    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .invoke_handler(tauri::generate_handler![get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
