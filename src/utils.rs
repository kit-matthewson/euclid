use macroquad::prelude::{screen_height, screen_width, Color, Vec2};

pub fn draw_circle(pos: Vec2, r: f32, colour: Color) {
    macroquad::prelude::draw_poly_lines(pos.x, pos.y, ((r + 10.0) / 2.0) as u8, r, 0.0, 2.0, colour)
}

pub fn draw_line(p1: Vec2, p2: Vec2, colour: Color) {
    let (a, b) = (p1.x, p1.y);
    let (m, n) = (p2.x, p2.y);

    fn valid(point: Vec2) -> bool {
        return point.x >= 0.0
            && point.x <= screen_width()
            && point.y >= 0.0
            && point.y <= screen_height();
    }

    let q1 = Vec2::new(((-b * (a - m)) / (b - n)) + a, 0.0);
    let q2 = Vec2::new(
        (((screen_height() - b) * (a - m)) / (b - n)) + a,
        screen_height(),
    );

    let q3 = Vec2::new(0.0, (((b - n) / (a - m)) * -a) + b);
    let q4 = Vec2::new(
        screen_width(),
        (((b - n) / (a - m)) * (screen_width() - a)) + b,
    );

    let possible = [q1, q2, q3, q4];
    let mut solutions = vec![];

    for point in possible {
        if valid(point) {
            solutions.push(point);
        }
    }

    if solutions.len() == 2 {
        let (m1, m2) = (solutions[0], solutions[1]);
        macroquad::prelude::draw_line(m1.x, m1.y, m2.x, m2.y, 2.0, colour);
    } else {
        macroquad::prelude::draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colour);
    }
}

pub fn draw_segment(p1: Vec2, p2: Vec2, colour: Color) {
    macroquad::prelude::draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colour)
}
