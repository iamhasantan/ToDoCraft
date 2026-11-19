#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

mod app;
mod theme;

use directories::ProjectDirs;
use eframe::egui;
use todocraft_storage::Store;

fn main() -> eframe::Result<()> {
    let _ = env_logger::try_init();
    let store = ProjectDirs::from("dev", "todocraft", "ToDoCraft")
        .ok_or_else(|| "Could not determine the application's data directory".to_owned())
        .and_then(|dirs| {
            Store::open(&dirs.data_local_dir().join("tasks.sqlite3")).map_err(|err| err.to_string())
        });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([750.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ToDoCraft",
        options,
        Box::new(move |cc| Ok(Box::new(app::ToDoCraftApp::new(cc, store)))),
    )
}
