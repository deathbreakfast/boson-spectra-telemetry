use spectra::spectra_schema;

spectra_schema! {
    BosonHandlerError {
        store: "boson",
        table: "boson_handler_error",
        version: "0.1.0",
        description: "Boson handler, Valence build, or persistence errors (no param payloads).",
        level: Error,
        fields: [
            task_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            job_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            run_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            error: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
