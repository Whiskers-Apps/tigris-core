use std::fs;

use serde::{Deserialize, Serialize};

use super::paths::get_settings_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_width")]
    pub width: u32,

    #[serde(default = "default_height")]
    pub height: u32,

    #[serde(default = "default_true")]
    pub show_recent_apps: bool,

    #[serde(default = "default_box_border_radius")]
    pub box_border_radius: u8,

    #[serde(default = "default_border_width")]
    pub border_width: u8,

    #[serde(default = "default_result_border_radius")]
    pub result_border_radius: u8,

    #[serde(default = "default_icon_border_radius")]
    pub icon_border_radius: u8,

    #[serde(default = "default_false")]
    pub hide_app_icons: bool,

    #[serde(default = "default_false")]
    pub accent_border: bool,

    #[serde(default = "default_true")]
    pub show_shortcut_hint: bool,

    #[serde(default = "default_shortcut_key")]
    pub shortcut_key: String,

    #[serde(default = "default_theme")]
    pub theme: Theme,

    #[serde(default = "default_extension_values")]
    pub extension_values: Vec<ExtensionValue>,

    #[serde(default = "default_search_engines")]
    pub search_engines: Vec<SearchEngine>,

    #[serde(default = "default_search_engine")]
    pub default_search_engine: usize,

    #[serde(default = "default_blacklist")]
    pub blacklist: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub accent: String,
    pub on_accent: String,
    pub danger: String,
    pub on_danger: String,
    pub background: String,
    pub secondary_background: String,
    pub tertiary_background: String,
    pub text: String,
    pub secondary_text: String,
    pub tertiary_text: String,
    pub disabled_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionValue {
    pub extension_id: String,
    pub setting_id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngine {
    pub id: usize,
    pub keyword: String,
    pub name: String,
    pub query: String,
}

// ===============================================================
// ===== Defaults
// ===============================================================

fn default_height() -> u32 {
    660
}

fn default_width() -> u32 {
    900
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_box_border_radius() -> u8 {
    16
}

fn default_result_border_radius() -> u8 {
    32
}

fn default_icon_border_radius() -> u8 {
    8
}

fn default_border_width() -> u8 {
    3
}

fn default_shortcut_key() -> String {
    String::from("alt")
}

fn default_theme() -> Theme {
    Theme {
        accent: String::from("#FFE072"),
        on_accent: String::from("#000000"),
        danger: String::from("#ff7272"),
        on_danger: String::from("#000000"),
        background: String::from("#141414"),
        secondary_background: String::from("#222222"),
        tertiary_background: String::from("#181818"),
        text: String::from("#f2f2f2"),
        secondary_text: String::from("#eaeaea"),
        tertiary_text: String::from("#d8d8d8"),
        disabled_text: String::from("#bdbdbd"),
    }
}

fn default_extension_values() -> Vec<ExtensionValue> {
    vec![]
}

fn default_search_engines() -> Vec<SearchEngine> {
    vec![
        SearchEngine {
            id: 0,
            keyword: String::from("ds"),
            name: String::from("DuckDuckGo"),
            query: String::from("https://duckduckgo.com/?q=%s"),
        },
        SearchEngine {
            id: 1,
            keyword: String::from("gs"),
            name: String::from("Google"),
            query: String::from("https://www.google.com/search?q=%s"),
        },
        SearchEngine {
            id: 2,
            keyword: String::from("bs"),
            name: String::from("Brave Search"),
            query: String::from("https://search.brave.com/search?q=%s"),
        },
        SearchEngine {
            id: 3,
            keyword: String::from("ss"),
            name: String::from("Startpage"),
            query: String::from("https://www.startpage.com/do/dsearch?q=%s"),
        },
    ]
}

fn default_search_engine() -> usize {
    0
}

fn default_blacklist() -> Vec<String> {
    vec![]
}

fn get_default_settings() -> Settings {
    Settings {
        width: default_width(),
        height: default_height(),
        show_recent_apps: true,
        box_border_radius: default_box_border_radius(),
        border_width: default_border_width(),
        result_border_radius: default_result_border_radius(),
        icon_border_radius: default_icon_border_radius(),
        hide_app_icons: false,
        accent_border: false,
        show_shortcut_hint: true,
        shortcut_key: default_shortcut_key(),
        theme: default_theme(),
        extension_values: default_extension_values(),
        search_engines: default_search_engines(),
        default_search_engine: default_search_engine(),
        blacklist: default_blacklist(),
    }
}

// ===============================================================
// ===== Methods
// ===============================================================

pub fn get_settings() -> Settings {
    let bytes = fs::read(get_settings_path()).unwrap_or(vec![]);
    let settings = bincode::deserialize(&bytes).unwrap_or(get_default_settings());
    settings
}

pub fn write_settings(settings: &Settings) {
    let bytes = bincode::serialize(settings).expect("Error serializing settings");
    fs::write(get_settings_path(), &bytes).expect("Error writing settings");
}

#[cfg(feature = "extension")]
pub fn get_extension_setting(extension_id: &str, setting_id: &str) -> Result<String, String> {
    let settings = get_settings();
    let extension_values = &settings.extension_values;

    for extension_value in extension_values {
        if extension_value.extension_id == extension_id && extension_value.setting_id == setting_id
        {
            return Ok(extension_value.value.to_owned());
        }
    }

    Err("Could not find setting with given values".to_owned())
}

#[cfg(feature = "extension")]
pub fn get_bool_extension_setting(extension_id: &str, setting_id: &str) -> Result<bool, String> {
    let value = get_extension_setting(extension_id, setting_id)?;

    Ok(if value == "true" { true } else { false })
}

#[cfg(feature = "extension")]
pub fn get_usize_extension_setting(extension_id: &str, setting_id: &str) -> Result<usize, String> {
    let value = get_extension_setting(extension_id, setting_id)?;

    if let Ok(number) = value.parse::<usize>() {
        return Ok(number);
    }

    Err("Error parsing setting to usize".to_string())
}
