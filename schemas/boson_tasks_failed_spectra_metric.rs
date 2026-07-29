use spectra::spectra_metric;

spectra_metric! {
    BosonTasksFailed {
        store: "boson",
        name: "boson_tasks_failed",
        version: "0.1.0",
        description: "Boson task runs failed. Labels: task_name, mode, reason.",
        level: Error,
    }
}
