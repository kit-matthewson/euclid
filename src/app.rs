use eframe::App;

use crate::{
    engine::{tools, Engine},
    ui,
};

pub struct Euclid {
    engine: Engine,

    tools: Vec<&'static dyn tools::Tool>,
}

impl App for Euclid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("new").clicked() {
                        self.engine = Engine::default();
                    }

                    if ui.button("save").clicked() {
                        todo!();
                    }

                    if ui.button("open").clicked() {
                        todo!();
                    }

                    if ui.button("quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("sidebar")
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui::grid::separator(ui);

                ui::grid::new("side-grid").show(ui, |ui| {
                    ui::grid::add_row(ui, "tool", |ui| {
                        egui::ComboBox::from_id_source("tool-select")
                            .selected_text(format!("{}", self.engine.current_tool.name()))
                            .show_ui(ui, |ui| {
                                for tool in &self.tools {
                                    ui.selectable_value(
                                        &mut self.engine.current_tool,
                                        *tool,
                                        tool.name(),
                                    );
                                }
                            });
                    });

                    ui::grid::text_row(
                        ui,
                        "operation",
                        self.engine
                            .current_tool
                            .instructions()
                            .get(self.engine.stats().num_points)
                            .unwrap_or(&""),
                    );

                    ui::grid::separator(ui);

                    ui::grid::text_row(ui, "current layer", &self.engine.current_layer.to_string());
                    ui::grid::text_row(ui, "current width", &self.engine.current_width.to_string());

                    ui::grid::separator(ui);

                    if let Some(cpu_usage) = frame.info().cpu_usage {
                        ui::grid::text_row(
                            ui,
                            "cpu time / ms",
                            &format!("{:.2}", cpu_usage * 1000.0),
                        );
                    }

                    ui::grid::add_struct(ui, self.engine.stats());
                });

                ui::grid::separator(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::plot::Plot::new("plot")
                    .allow_double_click_reset(false)
                    .show_x(false)
                    .show_y(false)
                    .x_axis_formatter(|_, _| String::new())
                    .y_axis_formatter(|_, _| String::new())
                    .data_aspect(1.0)
                    .legend(egui::plot::Legend::default())
                    .set_margin_fraction(egui::vec2(0.2, 0.2))
                    .show(ui, |ui| {
                        if ui.plot_clicked()
                            && ui.pointer_coordinate_drag_delta().length_sq() == 0.0
                        {
                            if let Some(point) = ui.pointer_coordinate() {
                                self.engine.click(point);
                            }
                        }

                        if ui.plot_secondary_clicked() {
                            self.engine.clear_points();
                        }

                        self.engine.show(ui);
                    });
            });
        });
    }
}

impl Default for Euclid {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
            tools: vec![
                &tools::Compass,
                &tools::StraightEdge,
                &tools::LineSegment,
                &tools::Arc,
            ],
        }
    }
}
