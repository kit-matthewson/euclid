use eframe::App;
use egui::{plot::Legend, Color32, Pos2};

use crate::{
    engine::{
        shapes::{CircleData, Construction, Shape},
        tools, Engine,
    },
    ui,
};

pub struct Euclid {
    engine: Engine,

    tools: Vec<&'static dyn tools::Tool>,

    point_inp: Pos2,
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

                ui::grid::new("side-grid").show(ui, |ui| {
                    ui::grid::add_text_row(
                        ui,
                        "cpu time / ms",
                        &format!("{:.2}", frame.info().cpu_usage.unwrap_or(0.0) * 1000.0),
                    );

                    ui::grid::add_text_row(
                        ui,
                        "constructions",
                        &self.engine.stats().num_constructions.to_string(),
                    );

                    ui::grid::add_text_row(
                        ui,
                        "intersections",
                        &self.engine.stats().num_intersections.to_string(),
                    );

                    ui::grid::separator(ui);

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

                    ui::grid::add_row(ui, "colour", |ui| {
                        egui::color_picker::color_edit_button_rgba(
                            ui,
                            &mut self.engine.current_color,
                            egui::color_picker::Alpha::OnlyBlend,
                        );
                    });

                    ui::grid::add_text_row(
                        ui,
                        "operation",
                        self.engine
                            .current_tool
                            .instructions()
                            .get(self.engine.stats().num_intersections)
                            .unwrap_or(&""),
                    );

                    ui::grid::separator(ui);

                    ui::grid::add_row(ui, "layer", |ui| {
                        egui::ComboBox::from_id_source("layer-select")
                            .selected_text(format!("{}", self.engine.current_layer))
                            .show_ui(ui, |ui| {
                                for i in 1..=5 {
                                    let name = format!("Layer {}", i);
                                    ui.selectable_value(
                                        &mut self.engine.current_layer,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    });

                    ui::grid::add_row(ui, "line width", |ui| {
                        ui.add(egui::Slider::new(&mut self.engine.current_width, 0.5..=5.0));
                    });

                    ui::grid::add_row(ui, "snap radius", |ui| {
                        ui.add(egui::Slider::new(
                            &mut self.engine.config.snap_radius,
                            0.0..=1.0,
                        ));
                    });

                    ui::grid::separator(ui);
                });

                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.point_inp.x));
                    ui.add(egui::DragValue::new(&mut self.point_inp.y));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("add point").clicked() {
                            self.engine.add_construction(Construction {
                                shape: Shape::Circle(CircleData {
                                    pos: self.point_inp,
                                    r: 0.0,
                                }),
                                layer: String::new(),
                                color: Color32::TRANSPARENT,
                                width: 0.0,
                                intersections: Vec::from([self.point_inp]),
                            });

                            self.point_inp = Pos2::ZERO;
                        };
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::plot::Plot::new("plot")
                    .allow_double_click_reset(false)
                    .show_x(false)
                    .show_y(false)
                    .legend(Legend::default())
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
            point_inp: Pos2::ZERO,

            tools: vec![
                &tools::Compass,
                &tools::StraightEdge,
                &tools::LineSegment,
                &tools::Arc,
            ],
        }
    }
}

impl Euclid {
    pub fn new() -> Self {
        Euclid::default()
    }
}
