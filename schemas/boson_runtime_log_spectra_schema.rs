use spectra::spectra_schema;

spectra_schema! {
    BosonRuntimeLog {
        store: "boson",
        table: "boson_runtime_log",
        version: "0.1.0",
        description: "Boson runtime boot and operational notes.",
        fields: [
            component: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            message: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            mode: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
        ],
    }
}
