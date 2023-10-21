use egui::{plot, Pos2};
use std::vec;

use super::{shapes, utils};

pub trait Tool {
    fn name(&self) -> &str;
    fn instructions(&self) -> Vec<&str>;
    fn num_points(&self) -> u8;
    fn get_guides(&self, points: &Vec<Pos2>, mouse: Pos2) -> Vec<plot::Line>;
    fn get_shape(&self, points: &Vec<Pos2>) -> shapes::Shape;
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

    fn get_guides(&self, points: &Vec<Pos2>, mouse: Pos2) -> Vec<plot::Line> {
        vec![utils::circle(points[0], points[0].distance(mouse))]
    }

    fn get_shape(&self, points: &Vec<Pos2>) -> shapes::Shape {
        shapes::Shape::Circle(shapes::CircleData {
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

    fn get_guides(&self, points: &Vec<Pos2>, mouse: Pos2) -> Vec<plot::Line> {
        vec![utils::line(points[0], mouse)]
    }

    fn get_shape(&self, points: &Vec<Pos2>) -> shapes::Shape {
        shapes::Shape::Line(shapes::LineData {
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

    fn get_guides(&self, points: &Vec<Pos2>, mouse: Pos2) -> Vec<plot::Line> {
        vec![utils::segment(points[0], mouse)]
    }

    fn get_shape(&self, points: &Vec<Pos2>) -> shapes::Shape {
        shapes::Shape::Segment(shapes::SegmentData {
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
        vec![
            "Select center",
            "Select radius",
            "Select start",
            "Select end",
        ]
    }

    fn num_points(&self) -> u8 {
        4
    }

    fn get_guides(&self, points: &Vec<Pos2>, mouse: Pos2) -> Vec<plot::Line> {
        if points.len() == 1 {
            return vec![
                utils::circle(points[0], points[0].distance(mouse)),
                utils::segment(points[0], mouse),
            ];
        } else if points.len() == 2 {
            return vec![
                utils::circle(points[0], points[0].distance(points[1])),
                utils::segment(
                    points[0],
                    points[0] + ((mouse - points[0]).normalized() * points[0].distance(points[1])),
                ),
            ];
        } else if points.len() == 3 {
            let r = points[0].distance(points[1]);

            return vec![
                utils::arc(
                    points[0],
                    r,
                    utils::arc_angle(points[2], points[0]),
                    utils::arc_angle(mouse, points[0]),
                ),
                utils::segment(
                    points[0],
                    points[0] + ((points[2] - points[0]).normalized() * r),
                ),
                utils::segment(
                    points[0],
                    points[0] + ((mouse - points[0]).normalized() * r),
                ),
            ];
        }

        Vec::new()
    }

    fn get_shape(&self, points: &Vec<Pos2>) -> shapes::Shape {
        shapes::Shape::Arc(shapes::ArcData {
            pos: points[0],
            r: points[0].distance(points[1]),
            start: utils::arc_angle(points[2], points[0]),
            stop: utils::arc_angle(points[3], points[0]),
        })
    }
}
