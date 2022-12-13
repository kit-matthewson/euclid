use std::f32::consts::PI;

use macroquad::prelude::*;

pub fn draw_circle(pos: Vec2, r: f32, color: Color, thickness: f32) {
    macroquad::prelude::draw_poly_lines(
        pos.x,
        pos.y,
        ((r + 10.0) / 2.0) as u8,
        r,
        0.0,
        thickness,
        color,
    )
}

pub fn draw_filled_circle(pos: Vec2, r: f32, color: Color) {
    macroquad::prelude::draw_poly(pos.x, pos.y, ((r + 10.0) / 2.0) as u8, r, 0.0, color)
}

pub fn draw_line(p1: Vec2, p2: Vec2, color: Color, thickness: f32) {
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
        macroquad::prelude::draw_line(m1.x, m1.y, m2.x, m2.y, thickness, color);
    } else {
        macroquad::prelude::draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
    }
}

pub fn draw_segment(p1: Vec2, p2: Vec2, color: Color, thickness: f32) {
    macroquad::prelude::draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
}

pub fn draw_arc(pos: Vec2, r: f32, start: f32, stop: f32, color: Color, thickness: f32) {
    fn point(pos: Vec2, r: f32, angle: f32) -> Vec2 {
        return Vec2::new(pos.x + (r * angle.cos()), pos.y + (r * angle.sin()));
    }

    let end = if start > stop { stop + 2.0 * PI } else { stop };

    let mut angle = start;
    let mut prev = point(pos, r, angle);

    while angle <= end {
        angle += 0.005;

        let a = point(pos, r, angle);
        macroquad::prelude::draw_line(a.x, a.y, prev.x, prev.y, thickness, color);
        prev = a;
    }
}

pub fn arc_angle(point: Vec2, centre: Vec2) -> f32 {
    let rel = (point - centre).normalize();

    let mut angle = libm::acos(Vec2::new(-1.0, 0.0).dot(rel) as f64) as f32;

    if angle.is_nan() {
        angle = 0.0;
    }

    let sign = if rel.y == 0.0 {
        1.0
    } else {
        rel.y / rel.y.abs()
    };

    return (angle * sign - PI).abs();
}

pub fn set_opacity(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}

pub fn within(v: f32, a: f32, b: f32) -> bool {
    return f32::min(a, b) <= v && v <= f32::max(a, b);
}

pub fn mod_inc(n: usize, max: usize) -> usize {
    return (n + 1) % max;
}

pub fn mod_dec(n: usize, max: usize) -> usize {
    return (n + (max - 1)) % max;
}
