//! Consumer-side forwarders onto typed Boson Spectra helpers.
//!
//! # Examples
//!
//! ```rust,no_run
//! use boson_spectra_telemetry::sink_forward::forward_counter;
//! use chrono::Utc;
//! use serde_json::json;
//!
//! forward_counter(
//!     "boson_tasks_enqueued",
//!     json!({"task_name": "send_email", "mode": "local"}),
//!     1,
//!     Utc::now(),
//! );
//! ```

use crate::helpers::{
    BosonHandlerErrorLogger, BosonRemoteErrorsRecorder, BosonRuntimeLogLogger,
    BosonTaskDurationMsRecorder, BosonTaskLogLogger, BosonTasksCompletedRecorder,
    BosonTasksEnqueuedRecorder, BosonTasksFailedRecorder,
};

fn field_str(fields: &serde_json::Value, key: &str) -> String {
    fields
        .get(key)
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Bool(b) => Some(b.to_string()),
            serde_json::Value::Number(n) => Some(n.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}

fn field_i64(fields: &serde_json::Value, key: &str) -> i64 {
    fields
        .get(key)
        .and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_i64(),
            serde_json::Value::String(s) => s.parse().ok(),
            _ => None,
        })
        .unwrap_or(0)
}

/// Forward a metric emit onto the matching typed recorder.
pub fn forward_counter(
    name: &str,
    labels: serde_json::Value,
    delta: i64,
    ts: chrono::DateTime<chrono::Utc>,
) {
    match name {
        "boson_remote_errors" => BosonRemoteErrorsRecorder::record_at(delta, labels, ts),
        "boson_task_duration_ms" => BosonTaskDurationMsRecorder::record_at(delta, labels, ts),
        "boson_tasks_completed" => BosonTasksCompletedRecorder::record_at(delta, labels, ts),
        "boson_tasks_enqueued" => BosonTasksEnqueuedRecorder::record_at(delta, labels, ts),
        "boson_tasks_failed" => BosonTasksFailedRecorder::record_at(delta, labels, ts),
        _ => {}
    }
}

/// Forward an event emit onto the matching typed logger.
pub fn forward_event(table: &str, fields: &serde_json::Value, ts: chrono::DateTime<chrono::Utc>) {
    match table {
        "boson_handler_error" => BosonHandlerErrorLogger::log_at(
            field_str(fields, "task_name"),
            field_str(fields, "job_id"),
            field_str(fields, "run_id"),
            field_str(fields, "error"),
            ts,
        ),
        "boson_runtime_log" => BosonRuntimeLogLogger::log_at(
            field_str(fields, "component"),
            field_str(fields, "message"),
            field_str(fields, "mode"),
            ts,
        ),
        "boson_task_log" => BosonTaskLogLogger::log_at(
            field_str(fields, "task_id"),
            field_str(fields, "job_id"),
            field_str(fields, "task_name"),
            field_i64(fields, "attempt"),
            field_str(fields, "pool"),
            field_str(fields, "mode"),
            field_i64(fields, "duration_ms"),
            field_str(fields, "status"),
            field_str(fields, "message"),
            ts,
        ),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn field_str_and_i64_happy_coercions() {
        let fields = json!({
            "s": "hello",
            "b": true,
            "n": 42,
            "ns": "7",
        });
        assert_eq!(field_str(&fields, "s"), "hello");
        assert_eq!(field_str(&fields, "b"), "true");
        assert_eq!(field_str(&fields, "n"), "42");
        assert_eq!(field_i64(&fields, "n"), 42);
        assert_eq!(field_i64(&fields, "ns"), 7);
    }

    #[test]
    fn field_str_and_i64_missing_or_invalid_default_sad() {
        let fields = json!({
            "arr": [],
            "obj": {},
            "bad": "not-a-number",
            "null": null,
        });
        assert_eq!(field_str(&fields, "missing"), "");
        assert_eq!(field_str(&fields, "arr"), "");
        assert_eq!(field_str(&fields, "obj"), "");
        assert_eq!(field_str(&fields, "null"), "");
        assert_eq!(field_i64(&fields, "missing"), 0);
        assert_eq!(field_i64(&fields, "bad"), 0);
        assert_eq!(field_i64(&fields, "arr"), 0);
        assert_eq!(field_i64(&fields, "null"), 0);
    }
}
