use eframe::App;
use egui::{
    plot::{Legend, PlotPoint},
    Pos2, RichText,
};

use crate::{
    engine::{tools, Engine},
    ui,
};

pub struct Euclid {
    engine: Engine,

    tools: Vec<&'static dyn tools::Tool>,

    point_inp: Pos2,
    show_axes: bool,
}

impl App for Euclid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            extreme_bg_color: self.engine.config.extreme_background_color.into(),
            faint_bg_color: self.engine.config.faint_background_color.into(),
            override_text_color: Some(self.engine.config.text_color.into()),
            ..Default::default()
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("new").clicked() {
                        self.engine.clear();
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

                ui.menu_button("examples", |ui| {
                    if ui.button("triangle").clicked() {
                        todo!()
                    }

                    if ui.button("pentagon").clicked() {
                        todo!()
                    }

                    if ui.button("hexagon").clicked() {
                        todo!()
                    }
                });
            });
        });

        egui::SidePanel::left("sidebar")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);

                ui::grid::new("side-grid").show(ui, |ui| {
                    ui::grid::add_text_row(
                        ui,
                        "cpu time / ms",
                        &format!("{:.2}", frame.info().cpu_usage.unwrap_or(0.0) * 1000.0),
                    );

                    ui::grid::add_struct(ui, self.engine.stats());

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
                        egui::ComboBox::from_id_source("color-select")
                            .selected_text(format!(
                                "{}",
                                self.engine
                                    .config
                                    .get_name(&self.engine.current_color)
                                    .unwrap_or("custom".to_owned())
                            ))
                            .show_ui(ui, |ui| {
                                for color in &self.engine.config.tool_colors {
                                    ui.selectable_value(
                                        &mut self.engine.current_color,
                                        *color,
                                        format!(
                                            "{:?}",
                                            self.engine.config.get_name(color).unwrap()
                                        ),
                                    );
                                }
                            });

                        egui::color_picker::color_edit_button_srgba(
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
                            .get(self.engine.stats().num_points)
                            .unwrap_or(&"none"),
                    );

                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.point_inp.x));
                        ui.add(egui::DragValue::new(&mut self.point_inp.y));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("insert point").clicked() {
                                self.engine
                                    .click(PlotPoint::new(self.point_inp.x, self.point_inp.y));
                                self.point_inp = Pos2::ZERO;
                            }
                        });
                    });

                    ui.end_row();

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
                        ui.add(egui::Slider::new(&mut self.engine.snap_radius, 0.0..=1.0));
                    });

                    ui::grid::add_row(ui, "show axes", |ui| {
                        ui.add(egui::Checkbox::new(&mut self.show_axes, ""));
                    });

                    ui::grid::add_row(ui, "show intersections", |ui| {
                        ui.add(egui::Checkbox::new(&mut self.engine.show_intersections, ""));
                    });

                    ui::grid::separator(ui);
                });

                ui.horizontal(|ui| {
                    ui.add_enabled_ui(self.engine.can_undo(), |ui| {
                        if ui.button("undo").clicked() {
                            self.engine.undo();
                        };
                    });

                    ui.add_enabled_ui(self.engine.can_redo(), |ui| {
                        if ui.button("redo").clicked() {
                            self.engine.redo();
                        };
                    });

                    if ui.button("clear").clicked() {
                        self.engine.clear();
                    };
                });

                ui::grid::separator(ui);

                let num = 5;
                for (i, construction) in
                    self.engine.constructions.iter().rev().take(num).enumerate()
                {
                    let text = RichText::new(format!("{}", construction))
                        .color(
                            self.engine
                                .config
                                .text_color
                                .gamma_multiply(1.0 - (i as f32 / num as f32)),
                        )
                        .font(egui::FontId::monospace(12.0));

                    ui.label(text);
                    ui.end_row();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().widgets.open.fg_stroke.color = self.engine.config.grid_color.into();
            ui.visuals_mut().widgets.open.weak_bg_fill = self.engine.config.background_color.into();
            ui.visuals_mut().faint_bg_color = self.engine.config.point_color.into();

            ui.vertical_centered(|ui| {
                egui::plot::Plot::new("plot")
                    .allow_double_click_reset(false)
                    .show_x(false)
                    .show_y(false)
                    .legend(Legend::default())
                    .data_aspect(1.0)
                    .legend(egui::plot::Legend::default())
                    .set_margin_fraction(egui::vec2(0.2, 0.2))
                    .show_axes([self.show_axes; 2])
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

impl Euclid {
    pub fn new() -> Self {
        Self {
            engine: Engine::new("config.yml"),
            point_inp: Pos2::ZERO,
            show_axes: true,

            tools: vec![
                &tools::Compass,
                &tools::StraightEdge,
                &tools::LineSegment,
                &tools::Arc,
            ],
        }
    }
}
