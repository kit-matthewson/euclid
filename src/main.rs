mod app;
mod engine;
mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    let _black = egui::Color32::from_rgba_premultiplied(10, 10, 10, 255);
    let _gray = egui::Color32::from_rgba_premultiplied(168, 154, 132, 150);
    let _white = egui::Color32::from_rgba_premultiplied(235, 219, 178, 255);
    let _red = egui::Color32::from_rgba_premultiplied(204, 36, 29, 255);
    let _green = egui::Color32::from_rgba_premultiplied(152, 151, 26, 255);
    let _yellow = egui::Color32::from_rgba_premultiplied(215, 153, 33, 255);
    let _blue = egui::Color32::from_rgba_premultiplied(69, 133, 136, 255);
    let _purple = egui::Color32::from_rgba_premultiplied(177, 98, 134, 255);

    // let config = euclid::EuclidConfig {
    //     padding: 12.0,

    //     background: black,
    //     foreground: white,
    //     guide: gray,
    //     highlight: yellow,

    //     tool_colors: vec![white, gray, red, green, yellow, blue, purple],

    //     font_size: 12,

    //     snap_radius: 15.0,
    //     line_thickness: 1.0,
    //     point_size: 2.0,
    // };

    eframe::run_native(
        "euclid",
        native_options,
        Box::new(|_cc| Box::new(app::Euclid::default())),
    )
}
