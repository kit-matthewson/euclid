mod app;
mod engine;
mod ui;
mod de;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "euclid",
        native_options,
        Box::new(|_cc| Box::new(app::Euclid::new())),
    )
}
