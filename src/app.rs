use eframe::App;

use crate::{engine::Engine, ui};

pub struct Euclid {
    engine: Engine,
}

impl Default for Euclid {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
        }
    }
}

impl App for Euclid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("new").clicked() {
                        todo!();
                    }

                    if ui.button("save").clicked() {
                        todo!();
                    }

                    if ui.button("open").clicked() {
                        todo!();
                    }

                    if ui.button("quit").clicked() {
                        todo!();
                    }
                });
            });
        });

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                if let Some(cpu_usage) = frame.info().cpu_usage {
                    ui::grid::add_row(ui, "cpu time / ms", &format!("{:.2}", cpu_usage * 1000.0));
                }

                ui::grid::separator(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::plot::Plot::new("graph")
                    .allow_double_click_reset(false)
                    .show_x(false)
                    .show_y(false)
                    .data_aspect(1.0)
                    .legend(egui::plot::Legend::default())
                    .set_margin_fraction(egui::vec2(0.2, 0.2))
                    .show(ui, |ui| {
                        if ui.plot_clicked()
                            && ui.pointer_coordinate_drag_delta().length_sq() == 0.0
                        {
                            if let Some(point) = ui.pointer_coordinate() {
                                self.engine.register_click(point);
                            }
                        }

                        self.engine.show(ui);
                    })
            });
        });
    }
}
