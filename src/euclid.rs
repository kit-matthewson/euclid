use macroquad::prelude::*;

use crate::{shapes::*, tool::*, utils};

pub struct ColorPalette {
    pub background: Color,
    pub foreground: Color,
    pub guide: Color,

    pub tool_a: Color,
    pub tool_b: Color,
    pub tool_c: Color,
    pub tool_d: Color,
    pub tool_e: Color,
}

struct Style {
    palette: ColorPalette,
    tool_colors: Vec<Color>,

    font: Font,
    font_size: u16,

    padding: f32,
}

pub struct Options {
    snap_radius: f32,
    line_thickness: f32,
    point_size: f32,

    show_interface: bool,
    show_intersections: bool,
    show_guides: bool,

    current_tool_index: usize,
    current_color_index: usize,
}

pub struct Euclid {
    constructions: Vec<Construction>,
    intersections: Vec<Vec2>,
    points: Vec<Vec2>,
    undo_queue: Vec<Construction>,
    tools: Vec<&'static dyn Tool>,

    style: Style,
    options: Options,
}

impl Euclid {
    pub fn new(palette: ColorPalette, font: Font) -> Euclid {
        Euclid {
            constructions: Vec::new(),
            intersections: Vec::new(),
            points: Vec::new(),
            undo_queue: Vec::new(),

            tools: vec![&Compass, &StraightEdge, &LineSegment, &Arc],

            style: Style {
                tool_colors: vec![
                    palette.foreground,
                    palette.guide,
                    palette.tool_a,
                    palette.tool_b,
                    palette.tool_c,
                    palette.tool_d,
                    palette.tool_e,
                ],

                palette,

                font,
                font_size: 12,

                padding: 8.0,
            },

            options: Options {
                show_interface: true,
                show_intersections: true,
                show_guides: true,

                snap_radius: 15.0,
                line_thickness: 1.0,
                point_size: 2.0,

                current_tool_index: 0,
                current_color_index: 0,
            },
        }
    }

    pub async fn run(&mut self) {
        loop {
            let mouse = Vec2::new(mouse_position().0, mouse_position().1);
            let mut snap_point = mouse;

            let mut max_distance = self.options.snap_radius;

            for intersection in self.intersections.iter() {
                if intersection.distance(mouse) <= max_distance {
                    snap_point = *intersection;
                    max_distance = intersection.distance(mouse);
                }
            }

            self.handle_input(snap_point);
            clear_background(self.style.palette.background);
            self.draw(mouse, snap_point);
            next_frame().await;
        }
    }

    fn draw(&self, mouse: Vec2, snap_point: Vec2) {
        self.draw_shapes(snap_point);

        if self.options.show_intersections {
            self.draw_intersections();
        }

        self.draw_points(mouse, snap_point);

        if self.options.show_interface {
            self.draw_interface();
        }
    }

    fn handle_input(&mut self, snap_point: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.points.push(snap_point);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.points.clear();
        }

        if self.points.len() as u8 == self.tools[self.options.current_tool_index].num_points() {
            let shape = self.tools[self.options.current_tool_index].get_construction(
                &self.points,
                self.style.tool_colors[self.options.current_color_index],
            );

            self.add_construction(shape);
            self.points.clear();
        }

        if mouse_wheel().1 < 0.0 {
            self.options.current_color_index = (self.options.current_color_index
                + (self.style.tool_colors.len() - 1))
                % self.style.tool_colors.len();
        } else if mouse_wheel().1 > 0.0 || is_key_pressed(KeyCode::LeftShift) {
            self.options.current_color_index =
                (self.options.current_color_index + 1) % self.style.tool_colors.len();
        }

        match get_last_key_pressed() {
            Some(KeyCode::Backspace) => {
                if !self.constructions.is_empty() {
                    let removed = self.constructions.pop().unwrap();

                    for other in self.constructions.iter() {
                        for _ in removed.shape.intersections(&other.shape) {
                            self.intersections.pop();
                        }
                    }

                    self.undo_queue.push(removed);
                }
            }

            Some(KeyCode::Tab) => {
                self.options.current_tool_index =
                    (self.options.current_tool_index + 1) % self.tools.len();
            }

            Some(KeyCode::F10) => {
                self.options.show_guides = !self.options.show_guides;
            }

            Some(KeyCode::F11) => {
                self.options.show_interface = !self.options.show_interface;
            }

            Some(KeyCode::F12) => {
                self.options.show_intersections = !self.options.show_intersections;
            }

            _ => (),
        };
    }

    fn draw_shapes(&self, snap_point: Vec2) {
        for construction in self.constructions.iter() {
            if self.options.show_guides || construction.color == self.style.palette.guide {
                construction.draw(self.options.line_thickness);
            }
        }

        if self.points.len() > 0 {
            self.tools[self.options.current_tool_index].draw_guide(
                &self.points,
                snap_point,
                utils::set_opacity(
                    self.style.tool_colors[self.options.current_color_index],
                    0.4,
                ),
                self.options.line_thickness,
            );
        }
    }

    fn draw_interface(&self) {
        let style = &self.style;

        let text_params = TextParams {
            font: style.font,
            font_size: style.font_size,
            color: style.palette.foreground,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
        };

        let text_height = measure_text("S", Some(style.font), style.font_size, 1.0).height;

        let mut y = style.padding + text_height;
        draw_text_ex("Euclid Geometry Engine", style.padding, y, text_params);

        let radius = 8.0;
        let line_space = 3.0;

        for (i, tool) in self.tools.iter().enumerate() {
            y = y + text_height + line_space;

            let text = if i == self.options.current_tool_index {
                format!("> {}", tool.name())
            } else {
                format!("  {}", tool.name())
            };

            draw_text_ex(&text, style.padding, y, text_params);
        }

        y = y + text_height + style.padding;

        draw_text_ex(
            &format!("Shapes: {}", self.constructions.len()),
            style.padding,
            y,
            text_params,
        );

        y = y + text_height + line_space;

        draw_text_ex(
            &format!("Intersections: {}", self.intersections.len()),
            style.padding,
            y,
            text_params,
        );

        for (i, color) in self.style.tool_colors.iter().enumerate() {
            let x = screen_width() / 2.0
                + (i as f32 - (self.style.tool_colors.len() as f32 / 2.0))
                    * (radius * 2.0 + self.style.padding);
            let mut y = screen_height() - self.style.padding - radius - radius;

            if i == self.options.current_color_index {
                y -= radius / 1.5;
            }

            utils::draw_filled_circle(Vec2::new(x, y), radius, *color);
        }
    }

    fn draw_points(&self, mouse: Vec2, snap_point: Vec2) {
        utils::draw_filled_circle(mouse, self.options.point_size, self.style.palette.guide);

        utils::draw_segment(
            mouse,
            snap_point,
            self.style.palette.guide,
            self.options.line_thickness,
        );

        utils::draw_filled_circle(
            snap_point,
            self.options.point_size,
            self.style.palette.tool_c,
        );

        for point in self.points.iter() {
            utils::draw_filled_circle(*point, self.options.point_size, self.style.palette.tool_c)
        }
    }

    fn draw_intersections(&self) {
        for intersection in self.intersections.iter() {
            utils::draw_filled_circle(
                *intersection,
                self.options.point_size,
                self.style.palette.tool_a,
            )
        }
    }

    pub fn add_construction(&mut self, construction: Construction) {
        for other in self.constructions.iter() {
            self.intersections
                .append(&mut construction.shape.intersections(&other.shape));
        }

        self.constructions.push(construction);
    }
}
