use egui::plot::{PlotPoint, Points};

#[derive(Debug)]
pub struct Engine {
    points: Vec<egui::Vec2>,
    intersections: Vec<PlotPoint>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct EngineStats {
    #[serde(rename = "number of points")]
    num_points: usize,
    #[serde(rename = "number of intersections")]
    num_intersections: usize,
}

impl EngineStats {
    pub fn from(engine: &Engine) -> EngineStats {
        EngineStats {
            num_points: engine.points.len(),
            num_intersections: engine.intersections.len(),
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            points: Vec::new(),
            intersections: Vec::new(),
        }
    }
}

impl Engine {
    pub fn show(&self, ui: &mut egui::plot::PlotUi) {
        ui.points(Points::new(
            self.points
                .iter()
                .map(|point| [point.x as f64, point.y as f64])
                .collect::<Vec<[f64; 2]>>(),
        ));
    }

    pub fn register_click(&mut self, point: PlotPoint) {
        self.points.push(point.to_vec2());
    }

    pub fn stats(&self) -> EngineStats {
        EngineStats::from(&self)
    }
}
