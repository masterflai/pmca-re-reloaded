//! The Tauri commands, one module per page or concern. `lib.rs` keeps the
//! shared state, the worker-thread helpers and the handler list.

mod analysis;
mod backup;
mod camera;
mod compare;
mod dialogs;
mod install;
mod keys;
mod network;
mod plugins;
mod settings;
mod shells;
mod sources;
mod tools;
mod tweaks;

pub(crate) use analysis::*;
pub(crate) use backup::*;
pub(crate) use camera::*;
pub(crate) use compare::*;
pub(crate) use dialogs::*;
pub(crate) use install::*;
pub(crate) use keys::*;
pub(crate) use network::*;
pub(crate) use plugins::*;
pub(crate) use settings::*;
pub(crate) use shells::*;
pub(crate) use sources::*;
pub(crate) use tools::*;
pub(crate) use tweaks::*;
