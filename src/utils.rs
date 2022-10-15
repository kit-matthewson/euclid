use macroquad::prelude::{Vec2, Color};

pub fn draw_circle(pos: Vec2, r: f32, colour: Color) {
    macroquad::prelude::draw_poly_lines(pos.x, pos.y, ((r + 10.0) / 2.0) as u8, r, 0.0, 1.0, colour)
}

pub fn draw_line(p1: Vec2, p2: Vec2, colour: Color) {
    macroquad::prelude::draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, colour);
}