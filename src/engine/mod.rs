pub mod config;
pub mod shapes;
pub mod tools;
pub mod utils;

use std::cmp::Ordering;

use egui::{
    plot::{PlotPoint, Points},
    Color32, Pos2, Rgba,
};

use self::{config::EngineConfig, shapes::Construction};

pub struct Engine {
    pub config: EngineConfig,

    points: Vec<Pos2>,
    constructions: Vec<Construction>,

    pub current_tool: &'static dyn tools::Tool,
    pub current_layer: String,
    pub current_color: Rgba,
    pub current_width: f32,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct EngineStats {
    #[serde(rename = "points")]
    pub num_points: usize,
    #[serde(rename = "constructions")]
    pub num_constructions: usize,
}

impl EngineStats {
    pub fn from(engine: &Engine) -> EngineStats {
        EngineStats {
            num_points: engine.points.len(),
            num_constructions: engine.constructions.len(),
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            config: EngineConfig::default(),
            points: Vec::new(),
            constructions: Vec::new(),

            current_tool: &tools::Compass,
            current_layer: String::from("Layer 1"),
            current_color: Rgba::WHITE,
            current_width: 1.0,
        }
    }
}

impl Engine {
    pub fn show(&self, ui: &mut egui::plot::PlotUi) {
        for construction in &self.constructions {
            ui.line(construction.get_line(ui));
            ui.points(
                Points::new(
                    construction
                        .intersections
                        .iter()
                        .map(|point| [point.x as f64, point.y as f64])
                        .collect::<Vec<[f64; 2]>>(),
                )
                .color(Color32::YELLOW)
                .name(&construction.layer),
            )
        }

        if let Some(mouse_pos) = ui.pointer_coordinate() {
            let mouse_pos = mouse_pos.to_pos2();

            let snap_pos = self.get_snap_pos(mouse_pos, self.config.snap_radius);

            if snap_pos != mouse_pos {
                ui.line(
                    utils::segment(mouse_pos, snap_pos).color(self.current_color.multiply(0.5)),
                );
            }

            if !self.points.is_empty() {
                for line in self.current_tool.get_guides(&self.points, snap_pos, &ui) {
                    ui.line(
                        line.color(self.current_color.multiply(0.5))
                            .width(self.current_width),
                    );
                }
            }
        }

        ui.points(
            Points::new(
                self.points
                    .iter()
                    .map(|point| [point.x as f64, point.y as f64])
                    .collect::<Vec<[f64; 2]>>(),
            )
            .color(Color32::RED),
        );
    }

    fn get_snap_pos(&self, mouse_pos: Pos2, snap_radius: f32) -> Pos2 {
        let mut snap_pos = self
            .constructions
            .iter()
            .flat_map(|con| con.intersections.clone())
            .min_by(|a, b| {
                if a.distance_sq(mouse_pos) < b.distance_sq(mouse_pos) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or(mouse_pos);

        if snap_pos.distance_sq(mouse_pos) > snap_radius * snap_radius {
            snap_pos = mouse_pos;
        }

        snap_pos
    }

    pub fn add_construction(&mut self, mut construction: Construction) {
        for other in self.constructions.iter() {
            construction
                .intersections
                .append(&mut construction.shape.intersections(&other.shape));
        }

        self.constructions.push(construction);
    }

    pub fn click(&mut self, point: PlotPoint) {
        self.points
            .push(self.get_snap_pos(point.to_pos2(), self.config.snap_radius));

        if self.points.len() as u8 == self.current_tool.num_points() {
            let shape = self.current_tool.get_shape(&self.points);

            let construction = Construction {
                shape,
                layer: self.current_layer.to_owned(),
                color: self.current_color.into(),
                width: self.current_width,
                intersections: Vec::new(),
            };

            self.add_construction(construction);
            self.points.clear();
        }
    }

    pub fn clear_points(&mut self) {
        self.points.clear();
    }

    pub fn stats(&self) -> EngineStats {
        EngineStats::from(&self)
    }
}
