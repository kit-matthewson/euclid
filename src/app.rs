use std::io::Write;

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

    show_save_window: bool,
    show_open_window: bool,

    file_name: Option<String>,
}

impl App for Euclid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            extreme_bg_color: self.engine.config.extreme_background_color,
            faint_bg_color: self.engine.config.faint_background_color,
            override_text_color: Some(self.engine.config.text_color),
            ..Default::default()
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("new").clicked() {
                        self.engine.clear();
                    }

                    if ui.button("save").clicked() {
                        self.show_save_window = true;
                    }

                    if ui.button("open").clicked() {
                        self.show_open_window = true;
                    }

                    if ui.button("quit").clicked() {
                        frame.close();
                    }
                });

                ui.menu_button("examples", |ui| {
                    let example_names = std::fs::read_dir("examples")
                        .expect("could not read examples folder")
                        .map(|res| res.map(|e| e.file_name()))
                        .collect::<Result<Vec<_>, std::io::Error>>()
                        .expect("could not read examples folder");

                    for example in example_names {
                        let example = example.to_string_lossy();
                        if ui
                            .button(example.clone().strip_suffix(".yml").unwrap())
                            .clicked()
                        {
                            let contents = std::fs::read_to_string(format!("examples/{}", example))
                                .expect("could not read file");

                            self.engine.load(&contents).expect("could not load file");
                        }
                    }
                });
            });
        });

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(16.0);

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
                            .selected_text(self.engine.current_tool.name().to_string())
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

                    ui::grid::add_row(ui, "color", |ui| {
                        egui::ComboBox::from_id_source("color-select")
                            .selected_text(
                                self.engine
                                    .config
                                    .get_name(&self.engine.current_color)
                                    .unwrap_or("custom".to_owned())
                                    .to_string(),
                            )
                            .show_ui(ui, |ui| {
                                for color in &self.engine.config.tool_colors {
                                    ui.selectable_value(
                                        &mut self.engine.current_color,
                                        *color,
                                        self.engine.config.get_name(color).unwrap().to_string(),
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
                            .selected_text(self.engine.current_layer.to_string())
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
            if ui::window::SaveWindow::new(&mut self.show_save_window, &mut self.file_name)
                .show(ctx)
            {
                let contents = self.engine.save().expect("could not save file");

                let mut file = std::fs::File::create(format!(
                    "saves/{}.yml",
                    self.file_name.clone().unwrap_or("unnamed".to_owned())
                ))
                .expect("could not create file");

                file.write_all(contents.as_bytes())
                    .expect("could not write save to file");

                self.show_save_window = false;
            }

            if ui::window::OpenWindow::new(&mut self.show_open_window, &mut self.file_name)
                .show(ctx)
            {
                let contents =
                    std::fs::read_to_string(format!("saves/{}.yml", self.file_name.clone().unwrap()))
                        .expect("could not read file");
                self.engine.load(&contents).expect("could not open file");

                self.show_open_window = false;
            }

            ui.vertical_centered(|ui| {
                ui.visuals_mut().widgets.open.fg_stroke.color = self.engine.config.grid_color;
                ui.visuals_mut().widgets.open.weak_bg_fill = self.engine.config.background_color;
                ui.visuals_mut().faint_bg_color = self.engine.config.point_color;

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

            show_save_window: false,
            show_open_window: false,

            file_name: None,

            tools: vec![
                &tools::Compass,
                &tools::StraightEdge,
                &tools::LineSegment,
                &tools::Arc,
            ],
        }
    }
}
