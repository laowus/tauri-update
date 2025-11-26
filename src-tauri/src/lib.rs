// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_app_info(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let version = app_handle.package_info().version.to_string();
    let name = app_handle.package_info().name.to_string();

    Ok(serde_json::json!({
        "version": version,
        "name": name
    }))
}

// 添加检查更新的命令
#[tauri::command]
async fn check_for_updates(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    use tauri_plugin_updater::UpdaterExt;

    let updater = app_handle.updater();
    match updater.check().await {
        Ok(update) => {
            if update.is_update_available() {
                Ok(serde_json::json!({
                    "update_available": true,
                    "current_version": update.current_version(),
                    "new_version": update.version(),
                    "body": update.body(),
                    "download_url": update.download_url()
                }))
            } else {
                Ok(serde_json::json!({
                    "update_available": false,
                    "current_version": update.current_version()
                }))
            }
        }
        Err(e) => Err(format!("检查更新失败: {}", e)),
    }
}

// 添加安装更新的命令
#[tauri::command]
async fn install_update(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    use tauri_plugin_updater::UpdaterExt;

    let updater = app_handle.updater();
    match updater.download_and_install().await {
        Ok(_) => Ok(serde_json::json!({
            "success": true,
            "message": "更新成功，应用将重启"
        })),
        Err(e) => Err(format!("安装更新失败: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_updater::Builder::new()
                .github(
                    tauri_plugin_updater::GithubReleaseBuilder::new()
                        .repo_owner("你的GitHub用户名")
                        .repo_name("你的仓库名称")
                        // 可选：指定资产模式，用于Windows
                        .asset_name("tauri-update_[platform]_x64[.ext]")
                        .build(),
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            check_for_updates,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
