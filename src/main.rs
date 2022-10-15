use std::fmt::format;

use macroquad::{miniquad::conf::Platform, prelude::*};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Euclid"),
        window_width: 0,
        window_height: 0,
        high_dpi: false,
        fullscreen: true,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Platform::default(),
    }
}

struct ColourPalette {
    black: Color,
    gray: Color,
    white: Color,
    yellow: Color,
}

enum Shape {
    Circle {
        pos: Vec2,
        r: f32,
        colour: Color,
    },

    Line {
        pos1: Vec2,
        pos2: Vec2,
        colour: Color,
    },
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("./assets/fonts/RobotoMono.ttf")
        .await
        .unwrap();

    let pallette = ColourPalette {
        black: Color::from_rgba(30, 30, 30, 255),
        gray: Color::from_rgba(127, 112, 97, 255),
        white: Color::from_rgba(230, 212, 163, 255),
        yellow: Color::from_rgba(204, 136, 26, 255),
    };

    let thickness = 1.0;

    let mut shapes: Vec<Shape> = Vec::new();

    let mut origin: Option<Vec2> = None;

    loop {
        clear_background(pallette.black);

        let mouse = Vec2::new(mouse_position().0, mouse_position().1);

        for shape in &shapes {
            match shape {
                Shape::Circle { pos, r, colour } => {
                    draw_circle(*pos, *r, *colour);
                }
                Shape::Line { pos1, pos2, colour } => {
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, *colour)
                }
            }
        }

        match origin {
            Some(pos) => {
                draw_circle(pos, 2.0, pallette.yellow);
                draw_circle(mouse, 2.0, pallette.yellow);

                draw_circle(pos, pos.distance(mouse), pallette.gray);
            }
            None => (),
        }

        draw_controls(&pallette, font).await;

        if is_mouse_button_pressed(MouseButton::Left) {
            match origin {
                Some(pos) => {
                    shapes.push(get_shape(pos, mouse, pallette.white));
                    origin = None;
                }
                None => origin = Some(mouse),
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            origin = None;
        }

        if is_key_pressed(KeyCode::Delete) {
            shapes.clear();
        } else if is_key_pressed(KeyCode::Backspace) {
            shapes.pop();
        }

        next_frame().await
    }
}

fn get_shape(pos1: Vec2, pos2: Vec2, colour: Color) -> Shape {
    Shape::Circle {
        pos: pos1,
        r: pos1.distance(pos2),
        colour,
    }
}

async fn draw_controls(_pallette: &ColourPalette, font: Font) {
    let padding = 8.0;

    let selected = 0;
    for (index, option) in vec!["Compass", "Straight Edge"].iter().enumerate() {
        let text = if index == selected {
            format!("> {}", option)
        } else {
            format!("  {}", option)
        };

        draw_text(&text, padding, 30.0 + (20.0 * (index as f32)), 16, font);
    }

    for (index, option) in vec!["White", "Yellow", "Red", "Green", "Blue"]
        .iter()
        .enumerate()
    {
        let text = if index == selected {
            format!("> {}", option)
        } else {
            format!("  {}", option)
        };

        draw_text(&text, padding, 100.0 + (20.0 * (index as f32)), 16, font);
    }

    draw_centred_text(
        "Euclid Geometry Engine",
        screen_width() / 2.0,
        padding + 8.0,
        16,
        font,
    );
}

fn draw_circle(pos: Vec2, r: f32, colour: Color) {
    draw_poly_lines(pos.x, pos.y, ((r + 10.0) / 2.0) as u8, r, 0.0, 1.0, colour)
}

fn draw_centred_text(text: &str, x: f32, y: f32, font_size: u16, font: Font) -> TextDimensions {
    let dimensions = measure_text(text, Some(font), font_size, 1.0);

    draw_text_ex(
        text,
        x - (dimensions.width / 2.0),
        y + dimensions.height,
        TextParams {
            font,
            font_size,
            ..Default::default()
        },
    );

    return dimensions;
}

fn draw_text(text: &str, x: f32, y: f32, font_size: u16, font: Font) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size,
            ..Default::default()
        },
    );
}
