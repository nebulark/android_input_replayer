#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use tokio::runtime::Runtime;

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    if true {
        // Log to stdout (if you run with `RUST_LOG=debug`).
        tracing_subscriber::fmt::init();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "ABD Utility",
            native_options,
            Box::new(|cc| Box::new(adb_util::AdbApp::new(cc))),
        );
    }
}