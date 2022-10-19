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
                Vec::new()
            } else {
                let m = (p2.x - p1.x) / (p1.y - p2.y);
                let c = ((p1.x * p1.x) + (p1.y * p1.y) - (p2.x * p2.x) - (p2.y * p2.y) - (r1 * r1) + (r2 * r2)) / (2.0 * (p1.y - p2.y));

                find_circle_intersections(p1, r1, &Shape::Line { points: [Vec2::new(0.0, c), Vec2::new(1.0, m + c)], colour: Color::default() })
            }
        },
        
        Shape::Line { points: [q1, q2], colour: _ } => {
            let m = (q1.y - q2.y) / (q1.x - q2.x);
            let c = -m * q1.x + q1.y;

            let d = ((m * m + 1.0) * (r1 * r1)) - f32::powi(p1.x * m - p1.y + c, 2);

            if d >= 0.0 {
                let t = p1.x + (p1.y * m) - (c * m);
                let u = 1.0 + (m * m);

                let x1 = (t - f32::sqrt(d)) / u;
                let x2 = (t + f32::sqrt(d)) / u;

                let y1 = m * x1 + c;
                let y2 = m * x2 + c;

                vec![Vec2::new(x1, y1), Vec2::new(x2, y2)]
            } else {
                Vec::new()
            }

        },
        Shape::LineSegment { points: _, colour: _ } => todo!(),
        Shape::Arc { points: _, colour: _ } => todo!(),
    }
}

fn find_line_intersection(p1: Vec2, p2: Vec2, b: &Shape) -> Vec<Vec2> {
    match b {
        Shape::Circle { pos, r, colour: _ } => find_circle_intersections(*pos, *r, &Shape::Line { points: [p1, p2], colour: Color::default() }),
        Shape::Line { points: [q1, q2], colour: _ } => {
            let m1 = (p1.y - p2.y) / (p1.x - p2.x);
            let m2 = (q1.y - q2.y) / (q1.x - q2.x);

            if m1 == m2 {
                return Vec::new();
            }

            let x = (q1.y - p1.y - (m2 * q1.x) + (m1 * p1.x)) / (m1 - m2);
            let y = m1 * (x - p1.x) + p1.y;

            vec![Vec2::new(x, y)]
        },

        Shape::LineSegment { points: _, colour: _ } => todo!(),
        Shape::Arc { points: _, colour: _ } => todo!(),
    }
}