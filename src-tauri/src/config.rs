use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};
use std::{env, fs};
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigObj {
    pub app_id: String,
    pub app_secret: String,
}

impl Default for ConfigObj {
    fn default() -> Self {
        Self {
            app_id: Default::default(),
            app_secret: Default::default(),
        }
    }
}

static CONFIG: LazyLock<RwLock<ConfigObj>> = LazyLock::new(|| RwLock::new(Default::default()));

pub fn load() {
    let path = config_file();
    let data = fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<ConfigObj>(&s).ok())
        .unwrap_or_default();
    *CONFIG.write().unwrap() = data;
}

pub fn save() -> Result<(), String> {
    let path = config_file();
    let json = serde_json::to_string_pretty(&*CONFIG.read().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

pub fn get_config() -> ConfigObj {
    CONFIG.read().unwrap().clone()
}

fn config_file() -> PathBuf {
    env::current_dir().unwrap().join("config.json")
}

#[tauri::command]
pub fn load_config(_app: AppHandle) -> ConfigObj {
    get_config()
}

#[tauri::command]
pub fn save_config(_app: AppHandle, config: ConfigObj) -> Result<(), String> {
    {
        *CONFIG.write().unwrap() = config;
    }
    save()
}
