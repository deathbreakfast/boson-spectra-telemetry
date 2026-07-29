//! Boson Spectra schema modules (inventory + typed helpers + topics).
//!
//! Each module wraps one `spectra_schema!` / `spectra_metric!` invocation under
//! `schemas/` at the repo root (relative to this file, one directory up from `src/`); the
//! macro generates the row/payload types, the typed logger/recorder, the Photon topic
//! constant, and the `inventory` registration for that table or counter/gauge. This module
//! itself is private — see [`crate::helpers`] and [`crate::topics`] for the re-exported,
//! effectively-public names.
#![allow(clippy::too_many_arguments, clippy::pedantic, clippy::nursery)]

/// `boson_handler_error` event schema (see `schemas/boson_handler_error_spectra_schema.rs`).
#[path = "../schemas/boson_handler_error_spectra_schema.rs"]
pub mod boson_handler_error;

/// `boson_remote_errors` counter schema (see `schemas/boson_remote_errors_spectra_metric.rs`).
#[path = "../schemas/boson_remote_errors_spectra_metric.rs"]
pub mod boson_remote_errors;

/// `boson_runtime_log` event schema (see `schemas/boson_runtime_log_spectra_schema.rs`).
#[path = "../schemas/boson_runtime_log_spectra_schema.rs"]
pub mod boson_runtime_log;

/// `boson_task_duration_ms` gauge schema (see `schemas/boson_task_duration_ms_spectra_metric.rs`).
#[path = "../schemas/boson_task_duration_ms_spectra_metric.rs"]
pub mod boson_task_duration_ms;

/// `boson_task_log` event schema (see `schemas/boson_task_log_spectra_schema.rs`).
#[path = "../schemas/boson_task_log_spectra_schema.rs"]
pub mod boson_task_log;

/// `boson_tasks_completed` counter schema (see `schemas/boson_tasks_completed_spectra_metric.rs`).
#[path = "../schemas/boson_tasks_completed_spectra_metric.rs"]
pub mod boson_tasks_completed;

/// `boson_tasks_enqueued` counter schema (see `schemas/boson_tasks_enqueued_spectra_metric.rs`).
#[path = "../schemas/boson_tasks_enqueued_spectra_metric.rs"]
pub mod boson_tasks_enqueued;

/// `boson_tasks_failed` counter schema (see `schemas/boson_tasks_failed_spectra_metric.rs`).
#[path = "../schemas/boson_tasks_failed_spectra_metric.rs"]
pub mod boson_tasks_failed;
