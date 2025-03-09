use std::path::PathBuf;

// =================================================================
// ==== Directories
// =================================================================

pub fn get_local_dir() -> PathBuf {
    return dirs::data_dir()
        .expect("Error getting data directory")
        .join("org-whiskersapps-tigris");
}

pub fn get_config_dir() -> PathBuf {
    return dirs::config_dir()
        .expect("Error getting config directory")
        .join("org-whiskersapps-tigris");
}

pub fn get_tmp_dir() -> PathBuf {
    return PathBuf::from("/tmp/tigris-launcher");
}

pub fn get_cache_dir() -> PathBuf {
    return dirs::cache_dir()
        .expect("Error getting cache dir")
        .join("org-whiskersapps-tigris");
}

pub fn get_assets_dir() -> PathBuf {
    return get_local_dir().join("assets");
}

pub fn get_icons_dir() -> PathBuf {
    return get_assets_dir().join("icons");
}

pub fn get_extensions_dir() -> PathBuf {
    return get_local_dir().join("extensions");
}

pub fn get_store_dir() -> PathBuf {
    return get_local_dir().join("store");
}

// =================================================================
// ==== Files
// =================================================================

pub fn get_apps_path() -> PathBuf {
    return get_cache_dir().join("apps.bin");
}

pub fn get_extensions_path() -> PathBuf {
    return get_cache_dir().join("extensions.bin");
}

pub fn get_recent_apps_path() -> PathBuf {
    return get_cache_dir().join("recent-apps.bin");
}

pub fn get_settings_path() -> PathBuf {
    return get_config_dir().join("settings.bin");
}

pub fn get_api_path() -> PathBuf {
    return get_tmp_dir().join("api.bin");
}

pub fn get_extensions_store_path() -> PathBuf {
    return get_store_dir().join("extensions.bin");
}

pub fn get_themes_store_path() -> PathBuf {
    return get_store_dir().join("themes.bin");
}
