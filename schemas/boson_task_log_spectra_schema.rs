use spectra::spectra_schema;

spectra_schema! {
    BosonTaskLog {
        store: "boson",
        table: "boson_task_log",
        version: "0.1.0",
        description: "Boson task run lifecycle trace (start, complete, fail, retry).",
        fields: [
            task_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            job_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            task_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            attempt: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            pool: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            mode: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            duration_ms: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            status: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            message: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
