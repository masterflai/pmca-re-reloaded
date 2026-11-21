//! The window's end of the macOS broker: sends the requests defined in
//! [`pmca_app::elevated`] to the separate `pmca-broker` executable and relays
//! its progress back to the window.
//!
//! [`pmca_app::broker`] carries the bytes; the broker's side is in
//! [`pmca_app::elevated`].

// Only macOS sends requests; elsewhere the window runs the session helpers itself.
#![cfg_attr(not(target_os = "macos"), allow(dead_code))]

pub(crate) use pmca_app::elevated::Request;

#[cfg(target_os = "macos")]
use crate::*;
#[cfg(target_os = "macos")]
use serde_json::Value;

/// Send `request` to the broker, starting it first if need be, and relay its
/// progress to the window.
#[cfg(target_os = "macos")]
pub(crate) fn call(
    app: &AppHandle,
    state: &AppState,
    request: Request,
) -> Result<Value, CommandError> {
    let mut guard = state.broker.lock().map_err(CommandError::internal)?;
    if guard.is_none() {
        *guard = Some(pmca_app::broker::Broker::start(&state.localizer()?)?);
    }
    relay(app, state, &mut guard, request)
}

/// [`call`] when the broker is already running; `None` otherwise, for the
/// requests that are not worth a password prompt.
#[cfg(target_os = "macos")]
pub(crate) fn call_if_running(
    app: &AppHandle,
    state: &AppState,
    request: Request,
) -> Result<Option<Value>, CommandError> {
    let mut guard = state.broker.lock().map_err(CommandError::internal)?;
    if guard.is_none() {
        return Ok(None);
    }
    relay(app, state, &mut guard, request).map(Some)
}

#[cfg(target_os = "macos")]
fn relay(
    app: &AppHandle,
    state: &AppState,
    broker: &mut Option<pmca_app::broker::Broker>,
    request: Request,
) -> Result<Value, CommandError> {
    let locale = state.locale.lock().map_err(CommandError::internal)?.clone();
    let envelope = serde_json::to_value(pmca_app::elevated::Envelope { locale, request })
        .map_err(CommandError::internal)?;
    let running = broker.as_mut().ok_or_else(|| CommandError::internal("no broker"))?;
    match running.call(&envelope, &mut progress(app)) {
        Ok(value) => Ok(value),
        Err(pmca_app::broker::CallError::Remote(e)) => Err(e.into()),
        Err(pmca_app::broker::CallError::Broken(e)) => {
            // Its sessions went with it; the next request starts a fresh one.
            broker.take();
            Err(e.into())
        }
    }
}
