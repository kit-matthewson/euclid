use macroquad::prelude::*;

#[allow(dead_code)]
pub enum Shape {
    Circle { pos: Vec2, r: f32, colour: Color },
    Line { points: [Vec2; 2], colour: Color },
    LineSegment { points: [Vec2; 2], colour: Color },
    Arc { points: [Vec2; 3], colour: Color },
}

pub fn find_intersection(a: &Shape, b: &Shape) -> Vec<Vec2> {
    match a {
        Shape::Circle { pos, r, colour: _ } => find_circle_intersections(*pos, *r, b),
        Shape::Line { points: [p1, p2], colour: _ } => find_line_intersection(*p1, *p2, b),
        Shape::LineSegment { points: _points, colour: _ } => todo!(),
        Shape::Arc { points: _points, colour: _ } => todo!(),
    }
}

fn find_circle_intersections(p1: Vec2, r1: f32, b: &Shape) -> Vec<Vec2> {
    match b {
        Shape::Circle { pos: p2, r: r2, colour: _ } => {
            if p1.distance_squared(*p2) > (r1 + r2) * (r1 + r2)  {
                vec![]
            } else {
                vec![p1]
            }
        },
        Shape::Line { points, colour } => todo!(),
        Shape::LineSegment { points, colour } => todo!(),
        Shape::Arc { points, colour } => todo!(),
    }
}

fn find_line_intersection(p1: Vec2, p2: Vec2, b: &Shape) -> Vec<Vec2> {
    match b {
        Shape::Circle { pos, r, colour } => find_circle_intersections(*pos, *r, b),
        Shape::Line { points, colour } => todo!(),
        Shape::LineSegment { points, colour } => todo!(),
        Shape::Arc { points, colour } => todo!(),
    }
}