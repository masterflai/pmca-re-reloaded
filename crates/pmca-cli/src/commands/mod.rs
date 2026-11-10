//! The subcommands, one module per group. `main.rs` parses the arguments and
//! dispatches here.

mod analyze;
mod backup;
mod camera;
mod compare;
mod network;
mod plugins;
mod projects;
mod service;
mod sources;
mod tweaks;
mod updater;

pub(crate) use analyze::*;
pub(crate) use backup::*;
pub(crate) use camera::*;
pub(crate) use compare::*;
pub(crate) use network::*;
pub(crate) use plugins::*;
pub(crate) use projects::*;
pub(crate) use service::*;
pub(crate) use sources::*;
pub(crate) use tweaks::*;
pub(crate) use updater::*;
