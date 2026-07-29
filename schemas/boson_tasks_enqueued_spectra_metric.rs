use spectra::spectra_metric;

spectra_metric! {
    BosonTasksEnqueued {
        store: "boson",
        name: "boson_tasks_enqueued",
        version: "0.1.0",
        description: "Boson tasks enqueued (new job inserted). Labels: task_name, mode.",
    }
}
