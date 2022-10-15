use macroquad::prelude::*;

#[allow(dead_code)]
pub enum Shape {
    Circle { pos: Vec2, r: f32, colour: Color },
    Line { points: [Vec2; 2], colour: Color },
    LineSegment { points: [Vec2; 2], colour: Color },
    Arc { points: [Vec2; 3], colour: Color },
}
