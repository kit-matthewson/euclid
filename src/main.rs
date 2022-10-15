use macroquad::{miniquad::conf::Platform, prelude::*};

mod utils;
mod shape;
mod tool;

use crate::shape::*;
use crate::tool::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Euclid"),
        window_width: 0,
        window_height: 0,
        high_dpi: false,
        fullscreen: true,
        sample_count: 4,
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

    let mut shapes: Vec<Shape> = Vec::new();
    let mut points: Vec<Vec2> = Vec::new();

    let tools: Vec<&dyn Tool> = vec![&Compass, &StraightEdge];
    let mut current_tool = 0;

    loop {
        clear_background(pallette.black);

        let mouse = Vec2::new(mouse_position().0, mouse_position().1);

        utils::draw_circle(mouse, 2.0, pallette.yellow);
        for point in points.iter() {
            utils::draw_circle(*point, 2.0, pallette.yellow);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            points.push(mouse);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            points.clear();
        }

        if is_key_pressed(KeyCode::Tab) {
            current_tool = (current_tool + 1) % tools.len();
        }

        if is_key_pressed(KeyCode::Delete) {
            shapes.clear();
        } else if is_key_pressed(KeyCode::Backspace) {
            shapes.pop();
        }

        if points.len() == tools[current_tool].num_points() as usize {
            let shape = tools[current_tool].get_shape(&points, pallette.white);
            shapes.push(shape);
            points.clear();
        } else if points.len() > 0 {
            tools[current_tool].draw_guide(&points, mouse, pallette.gray);
        }

        draw_shapes(&shapes);
        draw_interface(&pallette, font, &tools, current_tool);

        next_frame().await
    }
}

fn draw_shapes(shapes: &Vec<Shape>) {
    for shape in shapes {
        match shape {
            Shape::Circle { pos, r, colour } => {
                utils::draw_circle(*pos, *r, *colour);
            }

            Shape::Line {
                points: [pos1, pos2], colour,
            } => utils::draw_line(*pos1, *pos2, *colour),

            _ => (),
        }
    }
}

fn draw_interface(_pallette: &ColourPalette, font: Font, tools: &Vec<&dyn Tool>, selected_tool: usize) {
    let padding = 8.0;

    for (i, tool) in tools.iter().enumerate() {
        let text = if i == selected_tool {
            format!("> {}", tool.name())
        } else {
            format!("  {}", tool.name())
        };

        draw_text(&text, padding, 30.0 + (20.0 * (i as f32)), 16, font);
    }

    draw_centred_text(
        "Euclid Geometry Engine",
        screen_width() / 2.0,
        padding + 8.0,
        16,
        font,
    );
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
