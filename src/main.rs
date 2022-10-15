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
    white: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    purple: Color,
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("./assets/fonts/RobotoMono.ttf")
        .await
        .unwrap();

    let pallette = ColourPalette {
        black: Color::from_rgba(10, 10, 10, 255),
        white: Color::from_rgba(235, 219, 178, 255),
        red: Color::from_rgba(204, 36, 29, 255),
        green: Color::from_rgba(152, 151, 26, 255),
        yellow: Color::from_rgba(215, 153, 33, 255),
        blue: Color::from_rgba(69, 133, 136, 255),
        purple: Color::from_rgba(177, 98, 134, 255),
    };

    let mut shapes: Vec<Shape> = Vec::new();
    let mut points: Vec<Vec2> = Vec::new();

    let colours = vec![pallette.white, pallette.red, pallette.green, pallette.yellow, pallette.blue, pallette.purple];
    let mut current_colour = 0;

    let tools: Vec<&dyn Tool> = vec![&Compass, &StraightEdge];
    let mut current_tool = 0;

    loop {
        clear_background(pallette.black);

        let mouse = Vec2::new(mouse_position().0, mouse_position().1);

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

        if is_key_pressed(KeyCode::LeftShift) {
            current_colour = (current_colour + 1) % colours.len();
        }

        if is_key_pressed(KeyCode::Delete) {
            shapes.clear();
        } else if is_key_pressed(KeyCode::Backspace) {
            shapes.pop();
        }

        if points.len() == tools[current_tool].num_points() as usize {
            let shape = tools[current_tool].get_shape(&points, colours[current_colour]);
            shapes.push(shape);
            points.clear();
        } else if points.len() > 0 {
            tools[current_tool].draw_guide(&points, mouse, set_opacity(colours[current_colour], 0.4));
        }

        utils::draw_circle(mouse, 2.0, pallette.yellow);

        draw_shapes(&shapes);
        draw_interface(font, &pallette, &tools, current_tool, &colours, current_colour);

        next_frame().await
    }
}

fn set_opacity(colour: Color, a: f32) -> Color {
    Color::new(colour.r, colour.g, colour.b, a)
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

fn draw_interface(font: Font, pallette: &ColourPalette, tools: &Vec<&dyn Tool>, selected_tool: usize, colours: &Vec<Color>, selected_colour: usize) {
    let padding = 8.0;
    let font_size = 12.0;

    for (i, tool) in tools.iter().enumerate() {
        let x = padding;
        let y = 30.0 + (font_size * (i as f32));

        let text = if i == selected_tool {
            format!("> {}", tool.name())
        } else {
            format!("  {}", tool.name())
        };

        draw_text(&text, x, y, font_size as u16, font, pallette.white);
    }

    let radius = 8.0;

    for (i, colour) in colours.iter().enumerate() {
        let x = screen_width() / 2.0 + (i as f32 - (colours.len() as f32 / 2.0)) * (radius * 2.0 + padding);
        let mut y = screen_height() - padding - radius - radius;

        if i == selected_colour {
            y -= radius / 2.0;
        }

        draw_circle(x, y, radius, *colour);
    }

    draw_centred_text(
        "Euclid Geometry Engine",
        screen_width() / 2.0,
        padding + 8.0,
        16,
        font,
        pallette.white,
    );
}

fn draw_centred_text(text: &str, x: f32, y: f32, font_size: u16, font: Font, colour: Color) -> TextDimensions {
    let dimensions = measure_text(text, Some(font), font_size, 1.0);

    draw_text_ex(
        text,
        x - (dimensions.width / 2.0),
        y + dimensions.height,
        TextParams {
            font,
            font_size,
            color: colour,
            ..Default::default()
        },
    );

    return dimensions;
}

fn draw_text(text: &str, x: f32, y: f32, font_size: u16, font: Font, colour: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size,
            color: colour,
            ..Default::default()
        },
    );
}
