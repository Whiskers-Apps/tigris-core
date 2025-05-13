use std::{error::Error, fs, path::PathBuf};

use super::paths::get_extensions_dir;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Extension {
    pub id: String,

    pub name: String,

    pub description: String,

    #[serde(default = "default_none_string")]
    pub creator_name: Option<String>,

    #[serde(default = "default_none_string")]
    pub creator_link: Option<String>,

    #[serde(default = "default_none_string")]
    pub repository_link: Option<String>,

    pub settings: Vec<Setting>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Setting {
    pub id: String,

    pub name: String,

    pub description: String,

    pub value: String,

    pub setting_type: SettingType,

    #[serde(default = "default_none_usize")]
    pub min: Option<usize>,

    #[serde(default = "default_none_usize")]
    pub max: Option<usize>,

    #[serde(default = "default_none_usize")]
    pub step: Option<usize>,

    #[serde(default = "default_select_values")]
    pub select_values: Option<Vec<SelectValue>>,

    #[serde(default = "default_conditional_show")]
    pub conditional_show: Option<Vec<ConditionalShow>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SettingType {
    Text,
    Select,
    Switch,
    Slider,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelectValue {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConditionalShow {
    pub setting_id: String,
    pub setting_value: String,
}

fn default_none_string() -> Option<String> {
    None
}

fn default_none_usize() -> Option<usize> {
    None
}

fn default_select_values() -> Option<Vec<SelectValue>> {
    None
}

fn default_conditional_show() -> Option<Vec<ConditionalShow>> {
    None
}

// =================================================================
// ==== Methods
// =================================================================

pub fn get_extension_dir(extension_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    for entry in WalkDir::new(get_extensions_dir()?) {
        if let Ok(entry) = entry {
            let name = entry.file_name();

            if name == "manifest.json" {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(extension) = serde_json::from_str::<Extension>(&content) {
                        if extension.id == extension_id {
                            return Ok(entry.path().parent().unwrap().to_owned());
                        }
                    }
                }
            }
        }
    }

    Err("Could not find any extension with the given id".into())
}
