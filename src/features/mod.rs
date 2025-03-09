// General Code
#[cfg(feature = "default")]
pub mod paths;

#[cfg(feature = "default")]
pub mod search_results;

#[cfg(feature = "default")]
pub mod actions;

#[cfg(feature = "default")]
pub mod search;

#[cfg(feature = "default")]
pub mod api;

// Launcher Code
#[cfg(feature = "launcher")]
pub mod indexing;

#[cfg(feature = "default")]
pub mod settings;

#[cfg(feature = "launcher")]
pub mod apps;

// Shared Code
pub mod extensions;

pub mod utils;