mod bot;
mod config;
mod net;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    config::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            config::load_config,
            config::save_config,
            bot::fetch_bot_info,
            bot::list_panels,
            bot::add_panel,
            bot::update_panel,
            bot::delete_panel,
            net::get_public_ip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
