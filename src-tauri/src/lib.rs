#[tauri::command]
async fn get_app_info(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let version = app_handle.package_info().version.to_string();
    let name = app_handle.package_info().name.to_string();

    Ok(serde_json::json!({
        "version": version,
        "name": name
    }))
}

#[tauri::command]
async fn check_for_updates(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    use tauri_plugin_updater::UpdaterExt;

    // 首先解包updater的Result
    let updater = match app_handle.updater() {
        Ok(u) => u,
        Err(e) => return Err(format!("获取更新器失败: {}", e)),
    };

    // 打印调试信息
    println!("正在检查更新...");

    // 处理check方法返回的结果
    match updater.check().await {
        Ok(Some(update)) => {
            println!(
                "发现更新: 当前版本 {}, 新版本 {}",
                update.current_version, update.version
            );
            Ok(serde_json::json!({
                "update_available": true,
                "current_version": update.current_version,
                "new_version": update.version,
                "body": update.body.unwrap_or_default(),
                "download_url": update.download_url
            }))
        }
        Ok(None) => {
            let current_version = app_handle.package_info().version.to_string();
            println!("未发现更新，当前版本: {}", current_version);
            Ok(serde_json::json!({
                "update_available": false,
                "current_version": current_version
            }))
        }
        Err(e) => {
            let error_message = e.to_string();
            println!("更新检查错误: {}", error_message);

            // 捕获签名相关错误
            if error_message.contains("signature") {
                // 在开发环境中，可以模拟更新结果
                #[cfg(debug_assertions)]
                {
                    let current_version = app_handle.package_info().version.to_string();
                    println!("开发环境中忽略签名错误");
                    return Ok(serde_json::json!({
                        "update_available": true,
                        "current_version": current_version,
                        "new_version": "0.1.1", // 假设的新版本
                        "debug_message": "开发环境中模拟更新",
                        "error": error_message
                    }));
                }
            }
            Err(format!("检查更新失败: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_info, check_for_updates])
        .setup(|app| {
            #[cfg(desktop)]
            {
                // 打印插件配置信息
                println!("初始化updater插件...");
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                println!("updater插件初始化完成");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
