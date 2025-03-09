use std::fs;

use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use tux_icons::icon_fetcher::IconFetcher;
use walkdir::WalkDir;

use super::{
    apps::{get_recent_apps, write_apps, write_recent_apps, App},
    extensions::Extension,
    paths::{get_extensions_dir, get_extensions_path},
    settings::{get_settings, write_settings, ExtensionValue},
};

pub fn index_apps() {
    let mut apps = Vec::<App>::new();
    let icon_fetcher = IconFetcher::new().set_return_target_path(true);

    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                if !entry.no_display() && entry.type_().unwrap_or("") == "Application" {
                    let icon_path = icon_fetcher.get_icon_path_from_desktop(entry.path);
                    let exec_path = path.to_str().unwrap();
                    let name = entry.name(None).unwrap().to_string();

                    let mut app = App::new(exec_path, &name);

                    if let Some(icon_path) = icon_path {
                        if !icon_path.ends_with(".svgz") {
                            app = app.set_icon_path(&icon_path);
                        }
                    }

                    apps.push(app);
                }
            }
        }
    }

    apps.sort_by_key(|a| a.to_owned().name);

    let recent_apps = get_recent_apps()
        .iter()
        .map(|app| app.to_owned())
        .filter(|app| apps.contains(&app))
        .collect();

    write_apps(apps);
    write_recent_apps(recent_apps);
}

pub fn index_extensions() {
    let extensions_dir = get_extensions_dir();
    let mut extensions = Vec::<Extension>::new();
    let mut settings = get_settings();

    for entry in WalkDir::new(&extensions_dir) {
        if let Ok(entry) = entry {
            let name = entry.file_name();

            if name == "manifest.json" {
                if let Ok(content) = fs::read_to_string(&entry.path()) {
                    if let Ok(extension) = serde_json::from_str::<Extension>(&content) {
                        extensions.push(extension.to_owned());

                        let settings_values: Vec<ExtensionValue> = settings
                            .extension_values
                            .iter()
                            .filter(|setting| setting.extension_id == extension.id)
                            .map(|extension| extension.to_owned())
                            .collect();

                        for setting in &extension.settings {
                            if !settings_values
                                .iter()
                                .any(|value| value.setting_id == setting.id)
                            {
                                settings.extension_values.push(ExtensionValue {
                                    extension_id: extension.id.to_owned(),
                                    setting_id: setting.id.to_owned(),
                                    value: setting.value.to_owned(),
                                });
                            }
                        }

                        if !settings_values
                            .iter()
                            .any(|value| value.setting_id == "keyword")
                        {
                            settings.extension_values.push(ExtensionValue {
                                extension_id: extension.id.to_owned(),
                                setting_id: String::from("keyword"),
                                value: String::new(),
                            });
                        }
                    }
                }
            }
        }
    }

    write_settings(&settings);
    write_extensions(&extensions);
}

fn write_extensions(extensions: &Vec<Extension>) {
    let bytes = bincode::serialize(extensions).expect("Error serializing extensions");
    fs::write(get_extensions_path(), &bytes).expect("Error writing extensions");
}
