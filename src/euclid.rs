use macroquad::prelude::*;

use crate::{shapes::*, tool::*, utils};

pub struct Euclid {
    points: Vec<Vec2>,
    intersections: Vec<Vec2>,
    constructions: Vec<Construction>,
    undo_queue: Vec<Construction>,

    tools: Vec<&'static dyn Tool>,

    tool_i: usize,
    color_i: usize,
    layer_i: usize,

    show_interface: bool,
    show_intersections: bool,
    show_guides: bool,
    show_layer: [bool; 4],

    mouse: Vec2,
    snap_point: Vec2,
}

pub struct EuclidConfig {
    pub padding: f32,

    pub background: Color,
    pub foreground: Color,
    pub guide: Color,
    pub highlight: Color,
    pub tool_colors: Vec<Color>,

    pub font: Font,
    pub font_size: u16,

    pub snap_radius: f32,
    pub line_thickness: f32,
    pub point_size: f32,
}

impl Euclid {
    pub fn new() -> Euclid {
        Euclid {
            points: Vec::new(),
            intersections: Vec::new(),
            constructions: Vec::new(),
            undo_queue: Vec::new(),

            tools: vec![&Compass, &StraightEdge, &LineSegment, &Arc],

            tool_i: 0,
            color_i: 0,
            layer_i: 0,

            show_interface: true,
            show_intersections: true,
            show_guides: true,

            show_layer: [true; 4],
            mouse: Vec2::default(),
            snap_point: Vec2::default(),
        }
    }

    pub async fn run(&mut self, config: &EuclidConfig) {
        loop {
            clear_background(config.background);
            self.get_snap_point(config);
            self.handle_input(config);
            self.draw(config);
            next_frame().await;
        }
    }

    fn draw(&self, config: &EuclidConfig) {
        if self.show_interface {
            utils::draw_filled_circle(self.mouse, config.point_size, config.guide);

            utils::draw_segment(
                self.mouse,
                self.snap_point,
                config.guide,
                config.line_thickness,
            );

            utils::draw_filled_circle(self.snap_point, config.point_size, config.highlight);
        }

        self.draw_constructions(config);
        self.draw_guides(config);

        if self.show_intersections {
            self.draw_intersections(config);
        }

        self.draw_points(config);

        if self.show_interface {
            self.draw_interface(config);
        }
    }

    fn get_snap_point(&mut self, config: &EuclidConfig) {
        self.mouse = Vec2::new(mouse_position().0, mouse_position().1);

        let mut snap_point = self.mouse;
        let mut max_distance = config.snap_radius;

        for intersection in self.intersections.iter() {
            if intersection.distance(self.mouse) <= max_distance {
                snap_point = *intersection;
                max_distance = intersection.distance(self.mouse);
            }
        }

        self.snap_point = snap_point;
    }

    fn handle_input(&mut self, config: &EuclidConfig) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.points.push(self.snap_point);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.points.clear();
        }

        if self.points.len() as u8 == self.tools[self.tool_i].num_points() {
            let shape = self.tools[self.tool_i].get_shape(&self.points);

            let construction = Construction {
                shape,
                layer: self.layer_i,
                color: config.tool_colors[self.color_i],
            };

            self.add_construction(construction);
            self.points.clear();
        }

        if mouse_wheel().1 < 0.0 {
            self.color_i = utils::mod_dec(self.color_i, config.tool_colors.len());
        } else if mouse_wheel().1 > 0.0 {
            self.color_i = utils::mod_inc(self.color_i, config.tool_colors.len());
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

            Some(KeyCode::U) => match self.undo_queue.pop() {
                Some(construction) => self.add_construction(construction),
                None => (),
            },

            Some(KeyCode::Tab) => {
                if is_key_down(KeyCode::LeftShift) {
                    self.tool_i = utils::mod_dec(self.tool_i, self.tools.len());
                } else {
                    self.tool_i = utils::mod_inc(self.tool_i, self.tools.len());
                }
            }

            _ => (),
        };

        let last_key = get_last_key_pressed();

        if last_key.is_some() {
            let code = last_key.unwrap() as u8;

            if 69 <= code && code <= 72 {
                if self.points.len() as u8 > 1 {
                    self.points.clear();
                }

                self.tool_i = (code - 69) as usize;
            }

            if 73 <= code && code <= 76 {
                self.layer_i = (code - 73) as usize;
            }

            match last_key.unwrap() {
                KeyCode::F9 => {
                    self.show_intersections = !self.show_intersections;
                }

                KeyCode::F10 => {
                    self.show_guides = !self.show_guides;
                }

                KeyCode::F11 => {
                    self.show_interface = !self.show_interface;
                }

                KeyCode::F12 => {
                    self.show_layer[self.layer_i] = !self.show_layer[self.layer_i];
                }

                _ => (),
            }
        }
    }

    fn draw_constructions(&self, config: &EuclidConfig) {
        for construction in self.constructions.iter() {
            if self.show_guides || construction.color != config.guide {
                if self.show_layer[construction.layer] {
                    construction.draw(config.line_thickness);
                }
            }
        }
    }

    fn draw_guides(&self, config: &EuclidConfig) {
        if self.points.len() > 0 {
            self.tools[self.tool_i].draw_guide(
                &self.points,
                self.snap_point,
                utils::set_opacity(config.tool_colors[self.color_i], 0.4),
                config.line_thickness,
            );
        }
    }

    fn draw_interface(&self, config: &EuclidConfig) {
        let mut text_params = TextParams {
            font: config.font,
            font_size: config.font_size,
            color: config.foreground,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
        };

        fn list_options(
            config: &EuclidConfig,
            options: Vec<String>,
            selected: usize,
            y: &mut f32,
            line_height: f32,
            params: &mut TextParams,
            f_offset: usize,
        ) {
            for (i, option) in options.iter().enumerate() {
                if selected == i {
                    params.color = config.highlight;
                }

                let text = format!(" F{:2}: {}", i + f_offset, option);

                draw_text_ex(&text, config.padding, *y, *params);

                if i != options.len() - 1 {
                    *y = *y + line_height;
                }

                params.color = config.foreground;
            }
        }

        let text_height = measure_text("S", Some(config.font), config.font_size, 1.0).height;

        let radius = 8.0;
        let line_space = 3.0;

        let mut y = config.padding + text_height;

        let mut tool_names = Vec::new();
        for tool in self.tools.iter() {
            tool_names.push(String::from(tool.name()))
        }

        let mut layer_stats = Vec::new();
        for layer in self.show_layer {
            layer_stats.push(String::from(if layer { "Visible" } else { "Hidden" }))
        }

        draw_text_ex("Euclid Geometry Engine", config.padding, y, text_params);

        y = y + text_height + config.padding;

        draw_text_ex("Tools", config.padding, y, text_params);

        y = y + text_height + line_space;

        list_options(
            config,
            tool_names,
            self.tool_i,
            &mut y,
            text_height + line_space,
            &mut text_params,
            1,
        );

        y = y + text_height + config.padding;

        draw_text_ex("Layers", config.padding, y, text_params);

        y = y + text_height + line_space;

        list_options(
            config,
            layer_stats,
            self.layer_i,
            &mut y,
            text_height + line_space,
            &mut text_params,
            5,
        );

        y = y + text_height + config.padding;

        draw_text_ex("Show/Hide", config.padding, y, text_params);

        y = y + text_height + line_space;

        list_options(
            config,
            vec![
                String::from("Intersections"),
                String::from("Guides"),
                String::from("Interface"),
                String::from("Layer"),
            ],
            usize::MAX,
            &mut y,
            text_height + line_space,
            &mut text_params,
            9,
        );

        y = y + text_height + config.padding;

        draw_text_ex("Stats", config.padding, y, text_params);

        y = y + text_height + line_space;

        draw_text_ex(
            &format!("  Shapes: {}", self.constructions.len()),
            config.padding,
            y,
            text_params,
        );

        y = y + text_height + line_space;

        draw_text_ex(
            &format!("  Intersections: {}", self.intersections.len()),
            config.padding,
            y,
            text_params,
        );

        y = y + text_height + line_space;

        draw_text_ex(
            &format!(
                "  Frame time: {:.3}ms (fps:{:3})",
                macroquad::time::get_frame_time(),
                macroquad::time::get_fps()
            ),
            config.padding,
            y,
            text_params,
        );

        draw_text_ex(
            &(self.tools[self.tool_i].name().to_owned() + ":"),
            config.padding,
            screen_height() - config.padding - config.padding,
            text_params,
        );

        draw_text_ex(
            self.tools[self.tool_i].instructions()[self.points.len()],
            config.padding,
            screen_height() - config.padding - config.padding + text_height + line_space,
            text_params,
        );

        for (i, color) in config.tool_colors.iter().enumerate() {
            let x = (screen_width() / 2.0)
                + (i as f32 - ((config.tool_colors.len() - 1) as f32 / 2.0))
                    * (radius * 2.0 + config.padding);
            let mut y = screen_height() - config.padding - radius - radius;

            if i == self.color_i {
                y -= radius / 1.5;
            }

            utils::draw_filled_circle(Vec2::new(x, y), radius, *color);
        }
    }

    fn draw_points(&self, config: &EuclidConfig) {
        for point in self.points.iter() {
            utils::draw_filled_circle(*point, config.point_size, config.highlight);
        }
    }

    fn draw_intersections(&self, config: &EuclidConfig) {
        for intersection in self.intersections.iter() {
            utils::draw_filled_circle(*intersection, config.point_size, config.highlight)
        }
    }

    pub fn add_construction(&mut self, construction: Construction) {
        // Remove duplicates? Would me redoing undoing.

        for other in self.constructions.iter() {
            self.intersections
                .append(&mut construction.shape.intersections(&other.shape));
        }

        self.constructions.push(construction);
    }
}
