use spectra::spectra_metric;

spectra_metric! {
    BosonRemoteErrors {
        store: "boson",
        name: "boson_remote_errors",
        version: "0.1.0",
        description: "Boson remote HTTP coordinator failures. Labels: operation.",
        level: Error,
    }
}
