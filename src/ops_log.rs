//! Filtered remapping from [`boson_telemetry::OpsLog`] onto typed Spectra schemas.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::sanitize::sanitize_error_message;
use serde_json::{Map, Value};
use spectra_core::{try_log_event, try_record_counter, try_record_gauge};

const HIGH_CARDINALITY_METRIC_LABELS: &[&str] = &["task_name"];
const HIGH_CARDINALITY_EVENT_FIELDS: &[&str] = &["task_id", "job_id", "run_id"];
const EVENT_TEXT_FIELDS: &[&str] = &["error", "message"];

const ALLOWED_MODES: &[&str] = &["local", "distributed", "remote"];
const ALLOWED_FAILURE_REASONS: &[&str] =
    &["handler_error", "rate_limited", "persistence", "unknown"];

const fn counter_delta(value: f64) -> i64 {
    #[allow(clippy::cast_possible_truncation)]
    let delta = value.trunc() as i64;
    delta
}

fn hash_label_value(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("h{:016x}", hasher.finish())
}

fn allowlist_label_value(key: &str, value: &str) -> String {
    let normalized = value.trim().to_ascii_lowercase();
    let allowed = match key {
        "mode" => ALLOWED_MODES,
        "reason" => ALLOWED_FAILURE_REASONS,
        _ => return sanitize_metric_label(key, value),
    };
    if allowed.contains(&normalized.as_str()) {
        normalized
    } else {
        "unknown".to_owned()
    }
}

fn sanitize_metric_label(key: &str, value: &str) -> String {
    if HIGH_CARDINALITY_METRIC_LABELS.contains(&key) {
        hash_label_value(value)
    } else {
        value.to_owned()
    }
}

fn metric_label_keys(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "boson_tasks_enqueued" | "boson_tasks_completed" | "boson_task_duration_ms" => {
            Some(&["task_name", "mode"])
        }
        "boson_tasks_failed" => Some(&["task_name", "mode", "reason"]),
        "boson_remote_errors" => Some(&["operation"]),
        _ => None,
    }
}

fn filter_metric_labels(name: &str, labels: &[(&str, &str)]) -> Option<Vec<(String, String)>> {
    let allowed = metric_label_keys(name)?;
    let mut out = Vec::new();
    for (key, value) in labels {
        if allowed.contains(key) {
            out.push(((*key).to_owned(), allowlist_label_value(key, value)));
        }
    }
    Some(out)
}

fn event_field_keys(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "boson_handler_error" => Some(&["task_name", "job_id", "run_id", "error"]),
        "boson_runtime_log" => Some(&["component", "message", "mode"]),
        "boson_task_log" => Some(&[
            "task_id",
            "job_id",
            "task_name",
            "attempt",
            "pool",
            "mode",
            "duration_ms",
            "status",
            "message",
        ]),
        _ => None,
    }
}

fn sanitize_event_string_field(key: &str, text: &str) -> String {
    if EVENT_TEXT_FIELDS.contains(&key) {
        sanitize_error_message(text)
    } else if HIGH_CARDINALITY_EVENT_FIELDS.contains(&key) {
        hash_label_value(text)
    } else if key == "mode" {
        allowlist_label_value(key, text)
    } else {
        text.to_owned()
    }
}

fn sanitize_event_payload(name: &str, payload: &Value) -> Option<Value> {
    let allowed = event_field_keys(name)?;
    let object = payload.as_object()?;
    let mut filtered = Map::new();
    for key in allowed {
        if let Some(value) = object.get(*key) {
            let sanitized = match value {
                Value::String(text) => Value::String(sanitize_event_string_field(key, text)),
                other => other.clone(),
            };
            filtered.insert((*key).to_owned(), sanitized);
        }
    }
    Some(Value::Object(filtered))
}

/// Handle a [`boson_telemetry::OpsLog::record_counter`] call with allowlisted labels.
pub fn record_counter(name: &str, labels: &[(&str, &str)], value: f64) {
    let Some(filtered) = filter_metric_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_counter(name, &refs, counter_delta(value));
}

/// Handle a [`boson_telemetry::OpsLog::record_gauge`] call with allowlisted labels.
pub fn record_gauge(name: &str, labels: &[(&str, &str)], value: f64) {
    let Some(filtered) = filter_metric_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_gauge(name, &refs, value);
}

/// Handle a [`boson_telemetry::OpsLog::log_event`] call with schema-filtered payloads.
pub fn log_event(name: &str, payload: &Value) {
    let Some(filtered) = sanitize_event_payload(name, payload) else {
        return;
    };
    try_log_event(name, &filtered);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn counter_delta_integer_values_happy() {
        assert_eq!(counter_delta(0.0), 0);
        assert_eq!(counter_delta(1.0), 1);
        assert_eq!(counter_delta(42.0), 42);
        assert_eq!(counter_delta(-3.0), -3);
    }

    #[test]
    fn hash_label_value_is_stable_and_bounded() {
        let a = hash_label_value("send_email");
        let b = hash_label_value("send_email");
        let c = hash_label_value("other_task");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.starts_with('h'));
        assert_eq!(a.len(), 17);
    }

    #[test]
    fn filter_metric_labels_hashes_task_name_sad() {
        let filtered = filter_metric_labels(
            "boson_tasks_enqueued",
            &[("task_name", "demo"), ("mode", "local")],
        )
        .expect("known metric");
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].0, "task_name");
        assert_ne!(filtered[0].1, "demo");
        assert_eq!(filtered[1].0, "mode");
        assert_eq!(filtered[1].1, "local");
    }

    #[test]
    fn filter_metric_labels_unknown_mode_maps_to_unknown_sad() {
        let filtered = filter_metric_labels(
            "boson_tasks_failed",
            &[
                ("task_name", "t"),
                ("mode", "tenant-prod"),
                ("reason", "handler_error"),
            ],
        )
        .expect("known metric");
        assert_eq!(filtered[1].1, "unknown");
        assert_eq!(filtered[2].1, "handler_error");
    }

    #[test]
    fn unknown_metric_name_returns_none_sad() {
        assert!(filter_metric_labels("unknown_boson_metric", &[]).is_none());
    }

    #[test]
    fn sanitize_event_payload_hashes_ids_and_strips_unknown_fields_sad() {
        let payload = json!({
            "task_id": "run-abc",
            "job_id": "job-xyz",
            "task_name": "demo",
            "attempt": 1,
            "pool": "default",
            "mode": "local",
            "duration_ms": 3,
            "status": "failed",
            "message": "db failed password=hunter2",
            "secret": "drop-me"
        });
        let filtered = sanitize_event_payload("boson_task_log", &payload).expect("known event");
        assert_ne!(filtered["task_id"], "run-abc");
        assert_ne!(filtered["job_id"], "job-xyz");
        assert_eq!(filtered["task_name"], "demo");
        assert!(filtered.get("secret").is_none());
        assert!(filtered["message"]
            .as_str()
            .is_some_and(|m| m.contains("[redacted]")));
    }

    #[test]
    fn sanitize_handler_error_payload_hashes_run_id_sad() {
        let payload = json!({
            "task_name": "demo",
            "job_id": "job-1",
            "run_id": "run-1",
            "error": "handler boom"
        });
        let filtered =
            sanitize_event_payload("boson_handler_error", &payload).expect("known event");
        assert_ne!(filtered["job_id"], "job-1");
        assert_ne!(filtered["run_id"], "run-1");
        assert_eq!(filtered["error"], "handler boom");
    }
}
