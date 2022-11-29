use std::f32::consts::PI;

use crate::utils;

use macroquad::prelude::*;

pub struct Construction {
    pub shape: Shape,
    pub layer: usize,
    pub color: Color,
}

impl Construction {
    pub fn draw(&self, thickness: f32) {
        match &self.shape {
            Shape::Circle(circle) => {
                utils::draw_circle(circle.pos, circle.r, self.color, thickness)
            }

            Shape::Line(line) => utils::draw_line(line.p1, line.p2, self.color, thickness),

            Shape::Segment(segment) => {
                utils::draw_segment(segment.p1, segment.p2, self.color, thickness)
            }

            Shape::Arc(arc) => {
                utils::draw_arc(arc.pos, arc.r, arc.start, arc.stop, self.color, thickness)
            }
        }
    }
}

#[allow(dead_code)]
pub enum Shape {
    Circle(CircleData),
    Line(LineData),
    Segment(SegmentData),
    Arc(ArcData),
}

pub struct CircleData {
    pub pos: Vec2,
    pub r: f32,
}

pub struct LineData {
    pub p1: Vec2,
    pub p2: Vec2,
}

pub struct SegmentData {
    pub p1: Vec2,
    pub p2: Vec2,
}

pub struct ArcData {
    pub pos: Vec2,
    pub r: f32,
    pub start: f32,
    pub stop: f32,
}

impl SegmentData {
    pub fn line(&self) -> LineData {
        LineData {
            p1: self.p1,
            p2: self.p2,
        }
    }

    pub fn valid_points(&self, points: Vec<Vec2>) -> Vec<Vec2> {
        let mut valid = Vec::new();

        for point in points {
            if (self.p1.x < point.x && point.x < self.p2.x)
                || (self.p2.x < point.x && point.x < self.p1.x)
                    && (self.p1.y < point.y && point.y < self.p2.y)
                || (self.p2.y < point.y && point.y < self.p1.y)
            {
                valid.push(point);
            }
        }

        return valid;
    }
}

impl ArcData {
    pub fn circle(&self) -> CircleData {
        CircleData {
            pos: self.pos,
            r: self.r,
        }
    }

    pub fn valid_points(&self, points: Vec<Vec2>) -> Vec<Vec2> {
        let mut valid = Vec::new();

        for point in points {
            let angle = utils::arc_angle(point, self.pos);

            if self.stop < self.start {
                if (self.start <= angle && angle <= 2.0 * PI)
                    || (self.stop >= angle && angle >= 0.0)
                {
                    valid.push(point);
                }
            } else {
                if angle >= self.start && angle <= self.stop {
                    valid.push(point);
                }
            }
        }

        return valid;
    }
}

impl Shape {
    pub fn intersections(&self, other: &Shape) -> Vec<Vec2> {
        match self {
            Shape::Circle(a) => match other {
                Shape::Circle(b) => Shape::circle_circle(a, b),
                Shape::Line(b) => Shape::circle_line(a, b),
                Shape::Segment(b) => Shape::circle_segment(a, b),
                Shape::Arc(b) => Shape::circle_arc(a, b),
            },

            Shape::Line(a) => match other {
                Shape::Circle(b) => Shape::circle_line(b, a),
                Shape::Line(b) => Shape::line_line(a, b),
                Shape::Segment(b) => Shape::line_segment(a, b),
                Shape::Arc(b) => Shape::line_arc(a, b),
            },

            Shape::Segment(a) => match other {
                Shape::Circle(b) => Shape::circle_segment(b, a),
                Shape::Line(b) => Shape::line_segment(b, a),
                Shape::Segment(b) => Shape::segment_segment(a, b),
                Shape::Arc(b) => Shape::segment_arc(a, b),
            },

            Shape::Arc(a) => match other {
                Shape::Circle(b) => Shape::circle_arc(b, a),
                Shape::Line(b) => Shape::line_arc(b, a),
                Shape::Segment(b) => Shape::segment_arc(b, a),
                Shape::Arc(b) => Shape::arc_arc(a, b),
            },
        }
    }

    fn circle_circle(a: &CircleData, b: &CircleData) -> Vec<Vec2> {
        if a.pos.distance_squared(b.pos) > f32::powi(a.r + b.r, 2) {
            return Vec::new();
        } else {
            let m = (a.pos.x - b.pos.x) / (b.pos.y - a.pos.y);

            if m.is_infinite() {
                println!("[WARN] vertical circle-circle points");
                return Vec::new();
            }

            let c = ((a.pos.x * a.pos.x) + (a.pos.y * a.pos.y)
                - (b.pos.x * b.pos.x)
                - (b.pos.y * b.pos.y)
                - (a.r * a.r)
                + (b.r * b.r))
                / (2.0 * (a.pos.y - b.pos.y));

            return Shape::circle_line(
                a,
                &LineData {
                    p1: Vec2::new(0.0, c),
                    p2: Vec2::new(1.0, m + c),
                },
            );
        }
    }

    fn circle_line(a: &CircleData, b: &LineData) -> Vec<Vec2> {
        let m = (b.p1.y - b.p2.y) / (b.p1.x - b.p2.x);
        let c = -m * b.p1.x + b.p1.y;

        if m.is_infinite() {
            let x = b.p1.x;

            let pb = 2.0 * a.pos.y;
            let pc = f32::powi(x - a.pos.x, 2) + (a.pos.y * a.pos.y) - (a.r * a.r);

            let d = f32::sqrt((pb * pb) - (4.0 * pc));

            let y1 = (pb + d) / 2.0;
            let y2 = (pb - d) / 2.0;

            return vec![Vec2::new(x, y1), Vec2::new(x, y2)];
        }

        let d = ((m * m + 1.0) * (a.r * a.r)) - f32::powi(a.pos.x * m - a.pos.y + c, 2);

        if d >= 0.0 {
            let t = a.pos.x + (a.pos.y * m) - (c * m);
            let u = 1.0 + (m * m);

            let x1 = (t - f32::sqrt(d)) / u;
            let x2 = (t + f32::sqrt(d)) / u;

            let y1 = m * x1 + c;
            let y2 = m * x2 + c;

            return vec![Vec2::new(x1, y1), Vec2::new(x2, y2)];
        } else {
            return Vec::new();
        }
    }

    fn circle_segment(a: &CircleData, b: &SegmentData) -> Vec<Vec2> {
        let possible = Shape::circle_line(a, &b.line());
        return b.valid_points(possible);
    }

    fn circle_arc(a: &CircleData, b: &ArcData) -> Vec<Vec2> {
        let possible = Shape::circle_circle(a, &b.circle());
        return b.valid_points(possible);
    }

    fn line_line(a: &LineData, b: &LineData) -> Vec<Vec2> {
        let m1 = (a.p1.y - a.p2.y) / (a.p1.x - a.p2.x);
        let m2 = (b.p1.y - b.p2.y) / (b.p1.x - b.p2.x);

        if m1 == m2 {
            return Vec::new();
        }

        if m1.is_infinite() {
            let x = a.p1.x;
            let y = (m2 * x) - (m2 * b.p1.x) + b.p1.y;

            return vec![Vec2::new(x, y)];
        }

        if m2.is_infinite() {
            return Shape::line_line(b, a);
        }

        let x = (b.p1.y - a.p1.y - (m2 * b.p1.x) + (m1 * a.p1.x)) / (m1 - m2);
        let y = m1 * (x - a.p1.x) + a.p1.y;

        return vec![Vec2::new(x, y)];
    }

    fn line_segment(a: &LineData, b: &SegmentData) -> Vec<Vec2> {
        let possible = Shape::line_line(a, &b.line());
        return b.valid_points(possible);
    }

    fn line_arc(a: &LineData, b: &ArcData) -> Vec<Vec2> {
        let possible = Shape::circle_line(&b.circle(), a);
        return b.valid_points(possible);
    }

    fn segment_segment(a: &SegmentData, b: &SegmentData) -> Vec<Vec2> {
        let possible = Shape::line_segment(&a.line(), b);
        return a.valid_points(possible);
    }

    fn segment_arc(a: &SegmentData, b: &ArcData) -> Vec<Vec2> {
        let possible = Shape::line_arc(&a.line(), b);
        return a.valid_points(possible);
    }

    fn arc_arc(a: &ArcData, b: &ArcData) -> Vec<Vec2> {
        let possible = Shape::circle_arc(&a.circle(), b);
        return a.valid_points(possible);
    }
}
