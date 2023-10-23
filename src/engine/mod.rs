pub mod shapes;
pub mod tools;
pub mod utils;

use egui::{
    plot::{PlotPoint, Points},
    Color32, Pos2,
};

use self::shapes::Construction;

pub struct Engine {
    points: Vec<Pos2>,
    intersections: Vec<Pos2>,
    constructions: Vec<Construction>,

    pub current_tool: &'static dyn tools::Tool,
    pub current_layer: String,
    pub current_color: Color32,
    pub current_width: f32,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct EngineStats {
    #[serde(rename = "points")]
    pub num_points: usize,
    #[serde(rename = "intersections")]
    pub num_intersections: usize,
    #[serde(rename = "constructions")]
    pub num_constructions: usize,
}

impl EngineStats {
    pub fn from(engine: &Engine) -> EngineStats {
        EngineStats {
            num_points: engine.points.len(),
            num_intersections: engine.intersections.len(),
            num_constructions: engine.constructions.len(),
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            points: Vec::new(),
            intersections: Vec::new(),
            constructions: Vec::new(),

            current_tool: &tools::Compass,
            current_layer: String::from("Layer 1"),
            current_color: Color32::WHITE,
            current_width: 1.0,
        }
    }
}

impl Engine {
    pub fn show(&self, ui: &mut egui::plot::PlotUi) {
        for construction in &self.constructions {
            ui.line(construction.get_line());
        }

        if let Some(mouse_pos) = ui.pointer_coordinate() {
            if !self.points.is_empty() {
                for line in self
                    .current_tool
                    .get_guides(&self.points, mouse_pos.to_pos2())
                {
                    ui.line(line.color(self.current_color.gamma_multiply(0.5)));
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

        ui.points(
            Points::new(
                self.intersections
                    .iter()
                    .map(|point| [point.x as f64, point.y as f64])
                    .collect::<Vec<[f64; 2]>>(),
            )
            .color(Color32::YELLOW),
        );
    }

    pub fn add_construction(&mut self, construction: Construction) {
        for other in self.constructions.iter() {
            self.intersections
                .append(&mut construction.shape.intersections(&other.shape));
        }

        self.constructions.push(construction);
    }

    pub fn click(&mut self, point: PlotPoint) {
        self.points.push(point.to_pos2());

        if self.points.len() as u8 == self.current_tool.num_points() {
            let shape = self.current_tool.get_shape(&self.points);

            let construction = Construction {
                shape,
                layer: self.current_layer.to_owned(),
                color: self.current_color,
                width: self.current_width,
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
