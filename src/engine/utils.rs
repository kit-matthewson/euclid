use std::f32::consts::{PI, TAU};

use egui::{
    plot::{self, Line, PlotPoints},
    remap, Pos2,
};

pub fn circle(pos: Pos2, r: f32) -> plot::Line {
    let n = 512;
    let circle_points: PlotPoints = (0..=n)
        .map(|i| {
            let t = remap(i as f32, 0.0..=(n as f32), 0.0..=TAU);
            [(r * t.cos() + pos.x) as f64, (r * t.sin() + pos.y) as f64]
        })
        .collect();

    Line::new(circle_points)
}

pub fn line(p1: Pos2, p2: Pos2, top: f64, bottom: f64) -> plot::Line {
    let m = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64;
    let c = p1.y as f64 - (m * p1.x as f64);

    if m.is_infinite() {
        return Line::new(PlotPoints::new(vec![
            [p1.x as f64, top * 1.5],
            [p1.x as f64, bottom * 1.5],
        ]));
    }

    Line::new(PlotPoints::from_explicit_callback(
        move |x| (x * m) + c,
        ..,
        512,
    ))
}

pub fn segment(p1: Pos2, p2: Pos2) -> plot::Line {
    let n = 512;

    let line_points: PlotPoints = (0..=n)
        .map(|i| {
            let x = remap(i as f32, 0.0..=(n as f32), p1.x..=p2.x);
            let y = remap(i as f32, 0.0..=(n as f32), p1.y..=p2.y);
            [x as f64, y as f64]
        })
        .collect();

    Line::new(line_points)
}

pub fn arc(pos: Pos2, r: f32, a1: f32, a2: f32) -> plot::Line {
    let stop = if a1 > a2 { a2 + TAU } else { a2 };

    let n = 512;
    let arc_points: PlotPoints = (0..=n)
        .map(|i| {
            let t = remap(i as f32, 0.0..=(n as f32), a1..=stop);
            [(r * t.cos() + pos.x) as f64, (r * t.sin() + pos.y) as f64]
        })
        .collect();

    Line::new(arc_points)
}

pub fn arc_angle(point: Pos2, centre: Pos2) -> f32 {
    let rel = (point - centre).normalized();

    let mut angle = libm::acos(Pos2::new(-1.0, 0.0).to_vec2().dot(rel) as f64) as f32;

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

pub fn within(v: f32, a: f32, b: f32) -> bool {
    return f32::min(a, b) <= v && v <= f32::max(a, b);
}
