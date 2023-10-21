use egui::plot::{PlotPoint, Points};

#[derive(Debug)]
pub struct Engine {
    points: Vec<egui::Vec2>,
    _intersections: Vec<PlotPoint>,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            points: Vec::new(),
            _intersections: Vec::new(),
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
}
