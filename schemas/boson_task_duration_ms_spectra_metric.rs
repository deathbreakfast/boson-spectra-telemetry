use spectra::spectra_metric;

spectra_metric! {
    BosonTaskDurationMs {
        store: "boson",
        name: "boson_task_duration_ms",
        version: "0.1.0",
        description: "Boson task run wall-clock duration in milliseconds. Labels: task_name, mode.",
        level: Trace,
        default_sample_rate: 0.1,
    }
}
