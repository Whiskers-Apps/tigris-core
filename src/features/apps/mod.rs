use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use super::paths::{get_apps_path, get_recent_apps_path};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct App {
    pub path: String,
    pub icon_path: Option<String>,
    pub name: String,
}

impl App {
    pub fn new(path: &str, name: &str) -> Self {
        App {
            path: path.to_owned(),
            icon_path: None,
            name: name.to_owned(),
        }
    }

    pub fn set_icon_path(mut self, path: &Path) -> Self {
        let path_str = path.to_str().unwrap().to_owned();
        self.icon_path = Some(path_str);
        self
    }
}

pub fn get_apps() -> Vec<App> {
    let bytes = fs::read(get_apps_path()).unwrap_or(vec![]);
    let apps: Vec<App> = bincode::deserialize(&bytes).unwrap_or(vec![]);
    // let blacklist = get_settings().blacklist;
    //
    // let filtered_apps = apps
    //     .iter()
    //     .filter(|app| blacklist.contains(&app.name))
    //     .map(|app| app.to_owned())
    //     .collect();
    //
    // filtered_apps
    apps
}

pub fn write_apps(apps: Vec<App>) {
    let bytes = bincode::serialize(&apps).expect("Error serializing apps");
    fs::write(get_apps_path(), &bytes).expect("Error writing apps");
}

pub fn get_recent_apps() -> Vec<App> {
    let bytes = fs::read(get_recent_apps_path()).unwrap_or(vec![]);
    let apps: Vec<App> = bincode::deserialize(&bytes).unwrap_or(vec![]);
    apps
}

pub fn write_recent_apps(apps: Vec<App>) {
    let bytes = bincode::serialize(&apps).expect("Error serializing recent apps");
    fs::write(get_recent_apps_path(), &bytes).expect("Error writing recent apps");
}
