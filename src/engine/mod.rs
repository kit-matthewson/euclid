pub mod config;
pub mod shapes;
pub mod tools;
pub mod utils;

use egui::{
    plot::{LineStyle, PlotPoint, Points},
    Color32, Pos2,
};
use serde::Serialize;

use self::{config::EngineConfig, shapes::Construction};

enum RedoFrame {
    Single(Construction),
    Group(Vec<Construction>),
}

impl RedoFrame {
    fn constructions(&self) -> Vec<Construction> {
        match self {
            RedoFrame::Single(construction) => vec![construction.clone()],
            RedoFrame::Group(constructions) => constructions.clone(),
        }
    }
}

pub struct Engine {
    pub config: EngineConfig,

    pub points: Vec<Pos2>,
    pub constructions: Vec<Construction>,

    redo_stack: Vec<RedoFrame>,

    pub current_tool: &'static dyn tools::Tool,
    pub current_layer: String,
    pub current_color: Color32,
    pub current_width: f32,
    pub snap_radius: f32,
    pub show_intersections: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct EngineStats {
    #[serde(rename = "intersections")]
    pub num_intersections: usize,
    #[serde(rename = "points")]
    pub num_points: usize,
    #[serde(rename = "constructions")]
    pub num_constructions: usize,
}

impl EngineStats {
    pub fn from(engine: &Engine) -> EngineStats {
        EngineStats {
            num_intersections: engine
                .constructions
                .iter()
                .map(|con| con.intersections.len())
                .sum(),

            num_points: engine.points.len(),

            num_constructions: engine.constructions.len(),
        }
    }
}

impl Engine {
    pub fn new(config_file: &str) -> Self {
        let config = EngineConfig::read(config_file);

        Engine {
            config: config.clone(),

            points: Vec::new(),
            constructions: Vec::new(),

            redo_stack: Vec::new(),

            current_tool: &tools::Compass,
            current_layer: String::from("Layer 1"),
            current_color: *config.tool_colors.get(0).expect("no tools colors"),
            current_width: 1.0,
            snap_radius: 0.1,
            show_intersections: true,
        }
    }

    pub fn show(&self, ui: &mut egui::plot::PlotUi) {
        if self.points.is_empty() && self.constructions.is_empty() {
            return;
        }

        for construction in &self.constructions {
            ui.line(construction.get_line(ui));

            if self.show_intersections {
                ui.points(
                    Points::new(
                        construction
                            .intersections
                            .iter()
                            .map(|point| [point.x as f64, point.y as f64])
                            .collect::<Vec<[f64; 2]>>(),
                    )
                    .color(self.config.intersection_color)
                    .name(&construction.layer),
                );
            }
        }

        if let Some(mouse_pos) = ui.pointer_coordinate() {
            let mouse_pos = mouse_pos.to_pos2();
            let snap_pos = self.get_snap_pos(mouse_pos, self.snap_radius);

            if snap_pos != mouse_pos {
                ui.line(
                    utils::segment(mouse_pos, snap_pos)
                        .color(self.current_color.gamma_multiply(0.2))
                        .style(LineStyle::dotted_loose()),
                );
            } else if self.points.is_empty() {
                ui.line(
                    utils::circle(mouse_pos, self.snap_radius)
                        .color(self.current_color.gamma_multiply(0.2))
                        .style(LineStyle::dotted_loose()),
                );
            }

            if !self.points.is_empty() {
                for line in self.current_tool.get_guides(&self.points, snap_pos, ui) {
                    ui.line(
                        line.color(self.current_color.gamma_multiply(0.5))
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
            .color(self.config.point_color),
        );
    }

    fn get_snap_pos(&self, mouse_pos: Pos2, snap_radius: f32) -> Pos2 {
        let mut snap_pos = self
            .closest_intersection(mouse_pos, &self.points)
            .unwrap_or(mouse_pos);

        if snap_pos.distance_sq(mouse_pos) > snap_radius * snap_radius {
            snap_pos = mouse_pos;
        }

        snap_pos
    }

    pub fn closest_intersection(&self, mouse_pos: Pos2, ignore: &[Pos2]) -> Option<Pos2> {
        let mut closest: Option<Pos2> = None;

        for construction in &self.constructions {
            for intersection in construction
                .intersections
                .iter()
                .filter(|i| !ignore.contains(i))
            {
                if let Some(c) = closest {
                    if intersection.distance_sq(mouse_pos) < c.distance_sq(mouse_pos) {
                        closest = Some(*intersection);
                    }
                } else {
                    closest = Some(*intersection);
                }
            }
        }

        closest
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
            .push(self.get_snap_pos(point.to_pos2(), self.snap_radius));

        if self.points.len() as u8 == self.current_tool.num_points() {
            let shape = self.current_tool.get_shape(&self.points);

            let construction = Construction {
                shape,
                layer: self.current_layer.to_owned(),
                color: self.current_color,
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

    pub fn clear(&mut self) {
        self.redo_stack
            .push(RedoFrame::Group(self.constructions.clone()));
        self.constructions.clear();
        self.points.clear();
    }

    pub fn undo(&mut self) {
        if let Some(construction) = self.constructions.pop() {
            self.redo_stack.push(RedoFrame::Single(construction));
        }
    }

    pub fn redo(&mut self) {
        if let Some(redo_group) = self.redo_stack.pop() {
            for construction in redo_group.constructions() {
                self.constructions.push(construction);
            }
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.constructions.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn stats(&self) -> EngineStats {
        EngineStats::from(self)
    }
}
