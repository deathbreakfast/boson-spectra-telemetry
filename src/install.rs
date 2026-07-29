//! Install process-wide Boson ops log from `BOSON_TELEMETRY`.

use std::sync::Arc;

use boson_telemetry::{install_ops_log, ConsoleOpsLog, NoOpsLog, OpsLog};

use crate::ops_log;

/// Emits Boson ops metrics/events via `spectra-core` (non-recursive gate).
///
/// # Examples
///
/// ```rust,no_run
/// use boson_spectra_telemetry::SpectraOpsLog;
/// use boson_telemetry::OpsLog;
///
/// let log = SpectraOpsLog::new();
/// log.record_counter(
///     "boson_tasks_enqueued",
///     &[("task_name", "send_email"), ("mode", "local")],
///     1.0,
/// );
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct SpectraOpsLog;

impl SpectraOpsLog {
    /// New Spectra-backed ops log.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use boson_spectra_telemetry::SpectraOpsLog;
    ///
    /// let _log = SpectraOpsLog::new();
    /// ```
    pub const fn new() -> Self {
        Self
    }
}

impl OpsLog for SpectraOpsLog {
    fn record_counter(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        ops_log::record_counter(name, labels, value);
    }

    fn record_gauge(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        ops_log::record_gauge(name, labels, value);
    }

    fn log_event(&self, name: &str, payload: &serde_json::Value) {
        ops_log::log_event(name, payload);
    }
}

/// Install process-wide Boson ops log from `BOSON_TELEMETRY`.
///
/// # Examples
///
/// ```rust,no_run
/// boson_spectra_telemetry::install_ops_log_from_env();
/// ```
pub fn install_ops_log_from_env() {
    let log: Arc<dyn OpsLog> = match std::env::var("BOSON_TELEMETRY")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("off" | "0" | "false" | "none") => Arc::new(NoOpsLog),
        Some("console") => Arc::new(ConsoleOpsLog),
        _ => Arc::new(SpectraOpsLog::new()),
    };
    install_ops_log(log);
}
