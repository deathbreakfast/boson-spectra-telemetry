use spectra::spectra_metric;

spectra_metric! {
    BosonTasksCompleted {
        store: "boson",
        name: "boson_tasks_completed",
        version: "0.1.0",
        description: "Boson task runs completed successfully. Labels: task_name, mode.",
    }
}
