use macroquad::prelude::{Color, Vec2};

use crate::{shapes::*, utils};

pub trait Tool {
    fn name(&self) -> &str;
    fn num_points(&self) -> u8;
    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32);
    fn get_construction(&self, points: &Vec<Vec2>, color: Color) -> Construction;
}

pub struct Compass;
pub struct StraightEdge;
pub struct LineSegment;

impl Tool for Compass {
    fn name(&self) -> &str {
        "Compass"
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_circle(points[0], points[0].distance(mouse), color, thickness);
    }

    fn get_construction(&self, points: &Vec<Vec2>, color: Color) -> Construction {
        Construction {
            shape: Shape::Circle(CircleData {
                pos: points[0],
                r: points[0].distance(points[1]),
            }),

            color,
        }
    }
}

impl Tool for StraightEdge {
    fn name(&self) -> &str {
        "Straight Edge"
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_line(points[0], mouse, color, thickness);
    }

    fn get_construction(&self, points: &Vec<Vec2>, color: Color) -> Construction {
        Construction {
            shape: Shape::Line(LineData {
                p1: points[0],
                p2: points[1],
            }),

            color,
        }
    }
}

impl Tool for LineSegment {
    fn name(&self) -> &str {
        "Line Segment"
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_segment(points[0], mouse, color, thickness);
    }

    fn get_construction(&self, points: &Vec<Vec2>, color: Color) -> Construction {
        Construction {
            shape: Shape::Segment(SegmentData {
                p1: points[0],
                p2: points[1],
            }),

            color,
        }
    }
}
