use std::env;

use notify_rust::Notification;

pub fn on_wayland() -> bool {
    if let Ok(display_server) = env::var("XDG_SESSION_TYPE") {
        return display_server.to_lowercase() == "wayland";
    }

    false
}

pub fn send_notification(title: &str, description: &str) {
    Notification::new()
        .summary(&title)
        .body(&description)
        .icon("tigris")
        .show()
        .expect("Error sending notification");
}
