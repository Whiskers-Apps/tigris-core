use std::path::Path;

use serde::{Deserialize, Serialize};

use super::actions::ResultAction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub icon_path: Option<String>,
    pub icon_color: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub action: Option<ResultAction>,
}

impl SearchResult {
    pub fn new(title: &str) -> Self {
        SearchResult {
            icon_path: None,
            icon_color: None,
            title: title.to_owned(),
            description: None,
            action: None,
        }
    }

    pub fn set_icon_path(mut self, path: &Path) -> Self {
        let path_str = path.to_str().unwrap().to_owned();
        self.icon_path = Some(path_str);
        self
    }

    pub fn set_icon_color(mut self, color: &str) -> Self {
        self.icon_color = Some(color.to_owned());
        self
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn set_action(mut self, action: &ResultAction) -> Self {
        self.action = Some(action.to_owned());
        self
    }
}
