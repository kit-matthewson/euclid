use macroquad::prelude::*;

pub enum Shape {
    Circle { pos: Vec2, r: f32, colour: Color },
    Line { points: [Vec2; 2], colour: Color },
    LineSegment { points: [Vec2; 2], colour: Color },
    Arc { points: [Vec2; 3], colour: Color },
}

pub fn find_intersections(a: &Shape, b: &Shape) -> Vec<Vec2> {
    match a {
        Shape::Circle { pos, r, colour: _ } => find_circle_intersections(*pos, *r, b),

        Shape::Line {
            points: [p1, p2],
            colour: _,
        } => find_line_intersection(*p1, *p2, b),

        Shape::LineSegment {
            points: [p1, p2],
            colour: _,
        } => {
            let intersections = find_intersections(
                &Shape::Line {
                    points: [*p1, *p2],
                    colour: Color::default(),
                },
                b,
            );
            let mut valid = Vec::new();

            for intersection in intersections {
                if ((p1.x < intersection.x && intersection.x < p2.x)
                    || (p1.x > intersection.x && intersection.x > p2.x))
                    && ((p1.y < intersection.y && intersection.y < p2.y)
                        || (p1.y > intersection.y && intersection.y > p2.y))
                {
                    valid.push(intersection);
                }
            }

            return valid;
        }

        Shape::Arc {
            points: _points,
            colour: _,
        } => todo!(),
    }
}

fn find_circle_intersections(p1: Vec2, r1: f32, b: &Shape) -> Vec<Vec2> {
    match b {
        Shape::Circle {
            pos: p2,
            r: r2,
            colour: _,
        } => {
            if p1.distance_squared(*p2) > (r1 + r2) * (r1 + r2) {
                Vec::new()
            } else {
                let m = (p2.x - p1.x) / (p1.y - p2.y);

                if m.is_infinite() {
                    return find_circle_intersections(
                        p1,
                        r1,
                        &Shape::Line {
                            points: [Vec2::new(p1.x, 0.0), Vec2::new(p1.x, 1.0)],
                            colour: Color::default(),
                        },
                    );
                }

                let c = ((p1.x * p1.x) + (p1.y * p1.y) - (p2.x * p2.x) - (p2.y * p2.y) - (r1 * r1)
                    + (r2 * r2))
                    / (2.0 * (p1.y - p2.y));

                return find_circle_intersections(
                    p1,
                    r1,
                    &Shape::Line {
                        points: [Vec2::new(0.0, c), Vec2::new(1.0, m + c)],
                        colour: Color::default(),
                    },
                );
            }
        }

        Shape::Line {
            points: [q1, q2],
            colour: _,
        } => {
            let m = (q1.y - q2.y) / (q1.x - q2.x);
            let c = -m * q1.x + q1.y;

            if m.is_infinite() {
                let d = (-(p1.x * p1.x)) + (2.0 * p1.x * q1.x) + (r1 * r1) - (q1.x * q1.x);

                if d >= 0.0 {
                    let y1 = p1.y + f32::sqrt(d);
                    let y2 = p1.y - f32::sqrt(d);

                    return vec![Vec2::new(q1.x, y1), Vec2::new(q1.x, y2)];
                } else {
                    return Vec::new();
                }
            }

            let d = ((m * m + 1.0) * (r1 * r1)) - f32::powi(p1.x * m - p1.y + c, 2);

            if d >= 0.0 {
                let t = p1.x + (p1.y * m) - (c * m);
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

        Shape::LineSegment {
            points: _,
            colour: _,
        } => find_intersections(
            b,
            &Shape::Circle {
                pos: p1,
                r: r1,
                colour: Color::default(),
            },
        ),

        Shape::Arc {
            points: _,
            colour: _,
        } => todo!(),
    }
}

fn find_line_intersection(p1: Vec2, p2: Vec2, b: &Shape) -> Vec<Vec2> {
    match b {
        Shape::Circle { pos, r, colour: _ } => find_circle_intersections(
            *pos,
            *r,
            &Shape::Line {
                points: [p1, p2],
                colour: Color::default(),
            },
        ),

        Shape::Line {
            points: [q1, q2],
            colour: _,
        } => {
            let m1 = (p1.y - p2.y) / (p1.x - p2.x);
            let m2 = (q1.y - q2.y) / (q1.x - q2.x);

            if m1 == m2 {
                return Vec::new();
            }

            let mut x = (q1.y - p1.y - (m2 * q1.x) + (m1 * p1.x)) / (m1 - m2);
            let mut y = m1 * (x - p1.x) + p1.y;

            if m1.is_infinite() {
                x = p1.x;
                y = (m2 * x) - (m2 * q1.x) + q1.y;
            }

            if m2.is_infinite() {
                x = q1.x;
                y = (m1 * x) - (m1 * p1.x) + p1.y;
            }

            return vec![Vec2::new(x, y)];
        }

        Shape::LineSegment {
            points: _,
            colour: _,
        } => find_intersections(
            b,
            &Shape::Line {
                points: [p1, p2],
                colour: Color::default(),
            },
        ),

        Shape::Arc {
            points: _,
            colour: _,
        } => todo!(),
    }
}
