use std::{error::Error, fs, path::PathBuf};

// =================================================================
// ==== Directories
// =================================================================

pub fn get_local_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::data_dir()
        .ok_or_else(|| "Error getting local data dir".to_string())?
        .join("org-whiskersapps-tigris");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = dirs::config_dir()
        .ok_or_else(|| "Error getting config dir")?
        .join("org-whiskersapps-tigris");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_tmp_dir() -> PathBuf {
    return PathBuf::from("/tmp");
}

pub fn get_assets_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = get_local_dir()?.join("assets");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_icons_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = get_assets_dir()?.join("icons");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_extensions_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = get_local_dir()?.join("extensions");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

pub fn get_store_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = get_local_dir()?.join("store");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

// =================================================================
// ==== Files
// =================================================================

pub fn get_extensions_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_local_dir()?.join("indexing").join("extensions.bin"))
}

pub fn get_settings_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_config_dir()?.join("settings.bin"))
}

pub fn get_extensions_store_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_store_dir()?.join("extensions.bin"))
}

pub fn get_themes_store_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_store_dir()?.join("themes.bin"))
}

pub fn get_form_path() -> PathBuf {
    PathBuf::from("/tmp/tigris-form.bin")
}
