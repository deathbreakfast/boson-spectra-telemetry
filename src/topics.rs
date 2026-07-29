//! Transport `*Payload` / `*_TOPIC` DTOs from Boson Spectra schemas.
//!
//! Each `*_TOPIC` constant is the Photon topic name a Spectra sink publishes to, and the
//! matching `*Payload` is the serialized wire type carried on that topic.
//!
//! # Examples
//!
//! ```rust,no_run
//! use boson_spectra_telemetry::topics::{BosonTasksEnqueuedPayload, BOSON_TASKS_ENQUEUED_TOPIC};
//!
//! assert_eq!(BosonTasksEnqueuedPayload::topic(), BOSON_TASKS_ENQUEUED_TOPIC);
//! ```

/// Payload and topic constant for `boson_handler_error`.
pub use crate::schemas::boson_handler_error::{
    BosonHandlerErrorPayload, BOSON_HANDLER_ERROR_TOPIC,
};
/// Payload and topic constant for `boson_remote_errors`.
pub use crate::schemas::boson_remote_errors::{
    BosonRemoteErrorsPayload, BOSON_REMOTE_ERRORS_TOPIC,
};
/// Payload and topic constant for `boson_runtime_log`.
pub use crate::schemas::boson_runtime_log::{BosonRuntimeLogPayload, BOSON_RUNTIME_LOG_TOPIC};
/// Payload and topic constant for `boson_task_duration_ms`.
pub use crate::schemas::boson_task_duration_ms::{
    BosonTaskDurationMsPayload, BOSON_TASK_DURATION_MS_TOPIC,
};
/// Payload and topic constant for `boson_task_log`.
pub use crate::schemas::boson_task_log::{BosonTaskLogPayload, BOSON_TASK_LOG_TOPIC};
/// Payload and topic constant for `boson_tasks_completed`.
pub use crate::schemas::boson_tasks_completed::{
    BosonTasksCompletedPayload, BOSON_TASKS_COMPLETED_TOPIC,
};
/// Payload and topic constant for `boson_tasks_enqueued`.
pub use crate::schemas::boson_tasks_enqueued::{
    BosonTasksEnqueuedPayload, BOSON_TASKS_ENQUEUED_TOPIC,
};
/// Payload and topic constant for `boson_tasks_failed`.
pub use crate::schemas::boson_tasks_failed::{BosonTasksFailedPayload, BOSON_TASKS_FAILED_TOPIC};
