use crate::providers::addons::models::InstalledAddon;
use crate::providers::models::ProviderKind;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub auto_update: bool,
    pub last_update_check: u64,
    pub active_mode: String,
    pub active_provider: ProviderKind,
    pub active_theme: String,
    pub moviebox_enabled: bool,
    pub fourkhdhub_enabled: bool,
    pub dramachi_enabled: bool,
    pub bdix_circleftp_enabled: bool,
    pub bdix_dhakaflix_enabled: bool,
    pub bdix_probed: bool,
    pub streaming_enabled: bool,
    pub tv_enabled: bool,
    pub addons_enabled: bool,
    pub default_player: Option<String>,
    pub download_dir: Option<String>,
    pub vlc_path: Option<String>,
    pub mpv_path: Option<String>,
    pub iina_path: Option<String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            auto_update: true,
            last_update_check: 0,
            active_mode: "streaming".to_string(),
            active_provider: ProviderKind::MovieBox,
            active_theme: String::new(),
            moviebox_enabled: true,
            fourkhdhub_enabled: true,
            dramachi_enabled: true,
            bdix_circleftp_enabled: false,
            bdix_dhakaflix_enabled: false,
            bdix_probed: false,
            streaming_enabled: true,
            tv_enabled: true,
            addons_enabled: false,
            default_player: None,
            download_dir: None,
            vlc_path: None,
            mpv_path: None,
            iina_path: None,
        }
    }
}

pub const APP_NAME: &str = "moviezen-tui";
/// Pre-fork directory name. Used only to migrate user config/history from
/// an upstream MovieBox-Tui installation; never used for new files.
pub const LEGACY_APP_NAME: &str = "moviebox-tui";

/// Returns the value of the first environment variable in `names` that is set.
/// Used to prefer `MOVIEZEN_*` names while accepting legacy `MOVIEBOX_*` aliases.
pub fn env_var(names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| std::env::var(name).ok())
}

pub fn config_dir() -> Option<PathBuf> {
    if let Some(dir) = env_var(&["MOVIEZEN_CONFIG_DIR", "MOVIEBOX_CONFIG_DIR"]) {
        return Some(PathBuf::from(dir));
    }
    config_dir_for(APP_NAME)
}

fn config_dir_for(app: &str) -> Option<PathBuf> {
    if let Some(dir) = dirs::config_dir() {
        return Some(dir.join(app));
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        let p = PathBuf::from(xdg);
        if !p.as_os_str().is_empty() {
            return Some(p.join(app));
        }
    }
    if let Ok(prefix) = std::env::var("PREFIX") {
        let p = PathBuf::from(prefix).join("etc").join(app);
        if p.exists() {
            return Some(p);
        }
    }
    if let Some(dir) = dirs::home_dir().map(|h| h.join(".config").join(app)) {
        return Some(dir);
    }
    let fallback = std::env::temp_dir().join(app).join("config");
    log::warn!(
        "unable to locate user config directory, falling back to {}",
        fallback.display()
    );
    Some(fallback)
}

/// Old pre-fork config directory, if it exists on this system.
pub fn legacy_config_dir() -> Option<PathBuf> {
    config_dir_for(LEGACY_APP_NAME).filter(|dir| dir.exists())
}

pub fn data_dir() -> Option<PathBuf> {
    if let Some(dir) = env_var(&["MOVIEZEN_DATA_DIR", "MOVIEBOX_DATA_DIR"]) {
        return Some(PathBuf::from(dir));
    }
    data_dir_for(APP_NAME)
}

fn data_dir_for(app: &str) -> Option<PathBuf> {
    if let Some(dir) = dirs::data_dir() {
        return Some(dir.join(app));
    }
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        let p = PathBuf::from(xdg);
        if !p.as_os_str().is_empty() {
            return Some(p.join(app));
        }
    }
    if let Ok(prefix) = std::env::var("PREFIX") {
        let p = PathBuf::from(prefix).join("var").join("lib").join(app);
        if p.exists() {
            return Some(p);
        }
    }
    if let Some(dir) = dirs::home_dir().map(|h| h.join(".local").join("share").join(app)) {
        return Some(dir);
    }
    let fallback = std::env::temp_dir().join(app).join("data");
    log::warn!(
        "unable to locate user data directory, falling back to {}",
        fallback.display()
    );
    Some(fallback)
}

/// Old pre-fork data directory (watch history, favorites), if present.
pub fn legacy_data_dir() -> Option<PathBuf> {
    data_dir_for(LEGACY_APP_NAME).filter(|dir| dir.exists())
}

pub fn cache_dir() -> PathBuf {
    if let Some(dir) = env_var(&["MOVIEZEN_CACHE_DIR", "MOVIEBOX_CACHE_DIR"]) {
        return PathBuf::from(dir);
    }
    if let Some(dir) = dirs::cache_dir() {
        return dir.join(APP_NAME);
    }
    if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
        let p = PathBuf::from(xdg);
        if !p.as_os_str().is_empty() {
            return p.join(APP_NAME);
        }
    }
    if let Ok(prefix) = std::env::var("PREFIX") {
        let p = PathBuf::from(prefix)
            .join("var")
            .join("cache")
            .join(APP_NAME);
        if p.exists() {
            return p;
        }
    }
    dirs::home_dir()
        .map(|h| h.join(".cache").join(APP_NAME))
        .unwrap_or_else(|| std::env::temp_dir().join(APP_NAME))
}

pub fn logs_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(dir) = dirs::data_local_dir() {
            return dir.join(APP_NAME).join("logs");
        }
    }
    data_dir()
        .map(|dir| dir.join("logs"))
        .unwrap_or_else(|| std::env::temp_dir().join(APP_NAME).join("logs"))
}

pub fn scripts_dir() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("scripts"))
}

pub fn playback_state_dir() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("playback"))
}

pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.json"))
}

pub fn addons_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("addons_config.json"))
}

pub fn tv_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("tv_config.json"))
}

pub fn history_path() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("history.json"))
}

pub fn favorites_path() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("favorites.json"))
}

pub fn legacy_config_path() -> Option<PathBuf> {
    legacy_config_dir().map(|dir| dir.join("config.json"))
}

pub fn legacy_addons_path() -> Option<PathBuf> {
    legacy_config_dir().map(|dir| dir.join("addons_config.json"))
}

pub fn legacy_tv_path() -> Option<PathBuf> {
    legacy_config_dir().map(|dir| dir.join("tv_config.json"))
}

pub fn legacy_history_path() -> Option<PathBuf> {
    legacy_data_dir().map(|dir| dir.join("history.json"))
}

pub fn legacy_favorites_path() -> Option<PathBuf> {
    legacy_data_dir().map(|dir| dir.join("favorites.json"))
}

/// Pick which file to read: the new path when present, otherwise a legacy
/// pre-fork file. Returns `(path, needs_migration)`; `needs_migration` is true
/// when the legacy file was chosen and should be copied to the new location.
pub(crate) fn resolve_readable(
    new_path: Option<PathBuf>,
    legacy_path: Option<PathBuf>,
) -> Option<(PathBuf, bool)> {
    if let Some(path) = &new_path
        && path.exists()
    {
        return Some((path.clone(), false));
    }
    if let Some(path) = legacy_path
        && path.exists()
    {
        return Some((path, true));
    }
    new_path.map(|path| (path, false))
}

pub fn load() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };
    let Some((source, migrate)) = resolve_readable(Some(path), legacy_config_path()) else {
        return Config::default();
    };
    if source.exists() {
        if let Ok(content) = std::fs::read_to_string(&source) {
            let mut val: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("config invalid JSON: {e}; resetting");
                    serde_json::Value::default()
                }
            };
            let old_bdix = val
                .get("bdix_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if let Some(obj) = val.as_object_mut() {
                obj.remove("bdix_enabled");
            }
            if let Ok(mut config) = serde_json::from_value::<Config>(val) {
                if old_bdix {
                    config.bdix_circleftp_enabled = true;
                    config.bdix_dhakaflix_enabled = true;
                }
                if migrate {
                    log::info!("migrating legacy MovieBox-Tui config to MovieZen-TUI paths");
                    save(&config);
                }
                return config;
            }
        }
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let corrupt_path = source.with_extension(format!("corrupt.{stamp}"));
        log::error!(
            "failed to parse config from {}, rotating to {}",
            crate::logging::sanitize_path(&source),
            crate::logging::sanitize_path(&corrupt_path)
        );
        let _ = std::fs::rename(&source, corrupt_path);
    }
    Config::default()
}

pub fn save(config: &Config) {
    let Some(path) = config_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(config) {
        if let Err(error) = crate::cache::atomic_write_file(&path, json.as_bytes()) {
            log::warn!("failed to write config: {error}");
        }
    }
}

pub fn load_addons() -> Vec<InstalledAddon> {
    let resolved = resolve_readable(addons_path(), legacy_addons_path());
    let migrate = resolved.as_ref().is_some_and(|(_, needs)| *needs);
    let mut list = if let Some((source, _)) = resolved {
        if source.exists() {
            match std::fs::read_to_string(&source) {
                Ok(content) => match serde_json::from_str::<Vec<InstalledAddon>>(&content) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        let stamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        let corrupt_path = source.with_extension(format!("corrupt.{stamp}"));
                        log::error!(
                            "failed to parse addons config from {} ({e}), rotating to {}",
                            crate::logging::sanitize_path(&source),
                            crate::logging::sanitize_path(&corrupt_path)
                        );
                        let _ = std::fs::rename(&source, corrupt_path);
                        Vec::new()
                    }
                },
                Err(e) => {
                    log::warn!(
                        "failed to read addons config from {}: {e}",
                        crate::logging::sanitize_path(&source)
                    );
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    if !list.iter().any(|a| a.is_core()) {
        list.insert(0, InstalledAddon::cinemeta_default());
        save_addons(&list);
    } else {
        for a in &mut list {
            if a.is_core() {
                a.enabled = true;
            }
        }
        if migrate {
            log::info!("migrating legacy MovieBox-Tui addons config to MovieZen-TUI paths");
            save_addons(&list);
        }
    }
    list
}

    if !list.iter().any(|a| a.is_core()) {
        list.insert(0, InstalledAddon::cinemeta_default());
        save_addons(&list);
    } else {
        for a in &mut list {
            if a.is_core() {
                a.enabled = true;
            }
        }
    }
    list
}

pub fn save_addons(addons: &[InstalledAddon]) {
    let Some(path) = addons_path() else {
        return;
    };
    if let Some(app_dir) = path.parent()
        && std::fs::create_dir_all(app_dir).is_err()
    {
        return;
    }
    let Ok(json) = serde_json::to_string_pretty(addons) else {
        return;
    };
    if let Err(error) = crate::cache::atomic_write_file(&path, json.as_bytes()) {
        log::warn!("failed to write addons config: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_and_serde() {
        let config = Config::default();
        assert!(config.auto_update);
        assert_eq!(config.active_provider, ProviderKind::MovieBox);

        let json = serde_json::to_string(&config).expect("serialize");
        let deserialized: Config = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized.active_mode, config.active_mode);
    }
}
