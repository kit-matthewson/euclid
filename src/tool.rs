use macroquad::prelude::{Color, Vec2};

use crate::{shape::Shape, utils};

pub trait Tool {
    fn name(&self) -> &str;
    fn num_points(&self) -> i8;
    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, colour: Color);
    fn get_shape(&self, points: &Vec<Vec2>, colour: Color) -> Shape;
}

pub struct Compass;
pub struct StraightEdge;
pub struct LineSegment;

impl Tool for Compass {
    fn name(&self) -> &str {
        "Compass"
    }

    fn num_points(&self) -> i8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, colour: Color) {
        utils::draw_circle(points[0], points[0].distance(mouse), colour);
    }

    fn get_shape(&self, points: &Vec<Vec2>, colour: Color) -> Shape {
        Shape::Circle {
            pos: points[0],
            r: points[0].distance(points[1]),
            colour,
        }
    }
}

impl Tool for StraightEdge {
    fn name(&self) -> &str {
        "Straight Edge"
    }

    fn num_points(&self) -> i8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, colour: Color) {
        utils::draw_line(points[0], mouse, colour);
    }

    fn get_shape(&self, points: &Vec<Vec2>, colour: Color) -> Shape {
        Shape::Line {
            points: [points[0], points[1]],
            colour,
        }
    }
}

impl Tool for LineSegment {
    fn name(&self) -> &str {
        "Line Segment"
    }

    fn num_points(&self) -> i8 {
        2
    }

    fn draw_guide(&self, points: &Vec<Vec2>, mouse: Vec2, colour: Color) {
        utils::draw_segment(points[0], mouse, colour);
    }

    fn get_shape(&self, points: &Vec<Vec2>, colour: Color) -> Shape {
        Shape::LineSegment { points: [points[0], points[1]], colour: colour }
    }
}