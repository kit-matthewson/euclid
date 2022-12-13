use std::vec;

use macroquad::prelude::{Color, Vec2};

use crate::{shapes::*, utils};

pub trait Tool {
    fn name(&self) -> &str;
    fn instructions(&self) -> Vec<&str>;
    fn num_points(&self) -> u8;
    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32);
    fn get_shape(&self, points: &Vec<Vec2>) -> Shape;
}

pub struct Compass;
pub struct StraightEdge;
pub struct LineSegment;
pub struct Arc;

impl Tool for Compass {
    fn name(&self) -> &str {
        "Compass"
    }

    fn instructions(&self) -> Vec<&str> {
        vec!["Select center", "Select radius"]
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_circle(points[0], points[0].distance(mouse), color, thickness);
    }

    fn get_shape(&self, points: &Vec<Vec2>) -> Shape {
        Shape::Circle(CircleData {
            pos: points[0],
            r: points[0].distance(points[1]),
        })
    }
}

impl Tool for StraightEdge {
    fn name(&self) -> &str {
        "Straight Edge"
    }

    fn instructions(&self) -> Vec<&str> {
        vec!["Select first point", "Select second point"]
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_line(points[0], mouse, color, thickness);
    }

    fn get_shape(&self, points: &Vec<Vec2>) -> Shape {
        Shape::Line(LineData {
            p1: points[0],
            p2: points[1],
        })
    }
}

impl Tool for LineSegment {
    fn name(&self) -> &str {
        "Line Segment"
    }

    fn instructions(&self) -> Vec<&str> {
        vec!["Select start", "Select end"]
    }

    fn num_points(&self) -> u8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        utils::draw_segment(points[0], mouse, color, thickness);
    }

    fn get_shape(&self, points: &Vec<Vec2>) -> Shape {
        Shape::Segment(SegmentData {
            p1: points[0],
            p2: points[1],
        })
    }
}

impl Tool for Arc {
    fn name(&self) -> &str {
        "Arc"
    }

    fn instructions(&self) -> Vec<&str> {
        vec!["Select center", "Select radius", "Select start", "Select end"]
    }

    fn num_points(&self) -> u8 {
        4
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, color: Color, thickness: f32) {
        if points.len() == 1 {
            utils::draw_circle(points[0], points[0].distance(mouse), color, thickness);
            utils::draw_segment(points[0], mouse, color, thickness)
        } else if points.len() == 2 {
            utils::draw_circle(points[0], points[0].distance(points[1]), color, thickness);
            utils::draw_segment(
                points[0],
                points[0] + ((mouse - points[0]).normalize() * points[0].distance(points[1])),
                color,
                thickness,
            );
        } else if points.len() == 3 {
            let r = points[0].distance(points[1]);

            utils::draw_arc(
                points[0],
                r,
                utils::arc_angle(points[2], points[0]),
                utils::arc_angle(mouse, points[0]),
                color,
                thickness,
            );
            utils::draw_segment(
                points[0],
                points[0] + ((points[2] - points[0]).normalize() * r),
                color,
                thickness,
            );
            utils::draw_segment(
                points[0],
                points[0] + ((mouse - points[0]).normalize() * r),
                color,
                thickness,
            );
        }
    }

    fn get_shape(&self, points: &Vec<Vec2>) -> Shape {
        Shape::Arc(ArcData {
            pos: points[0],
            r: points[0].distance(points[1]),
            start: utils::arc_angle(points[2], points[0]),
            stop: utils::arc_angle(points[3], points[0]),
        })
    }
}
