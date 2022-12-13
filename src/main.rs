mod euclid;
mod shapes;
mod tool;
mod utils;

use std::fs;

use euclid::*;
use macroquad::{miniquad::conf::Platform, prelude::*};
use shapes::{Construction, SegmentData};
use yaml_rust::YamlLoader;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Euclid"),
        window_width: 0,
        window_height: 0,
        high_dpi: true,
        fullscreen: true,
        sample_count: 1,
        window_resizable: true,
        icon: None,
        platform: Platform::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let config_str = fs::read_to_string("config.yml").expect("could not read config.yml");
    let _config = YamlLoader::load_from_str(&config_str).expect("could not parse config yaml");

    let roboto = load_ttf_font("./assets/fonts/RobotoMono.ttf")
        .await
        .expect("failed to load font");

    let black = Color::from_rgba(10, 10, 10, 255);
    let gray = Color::from_rgba(168, 154, 132, 150);
    let white = Color::from_rgba(235, 219, 178, 255);
    let red = Color::from_rgba(204, 36, 29, 255);
    let green = Color::from_rgba(152, 151, 26, 255);
    let yellow = Color::from_rgba(215, 153, 33, 255);
    let blue = Color::from_rgba(69, 133, 136, 255);
    let purple = Color::from_rgba(177, 98, 134, 255);

    let config = EuclidConfig {
        padding: 12.0,

        background: black,
        foreground: white,
        guide: gray,
        highlight: yellow,

        tool_colors: vec![white, gray, red, green, yellow, blue, purple],

        font: roboto,
        font_size: 12,

        snap_radius: 15.0,
        line_thickness: 1.0,
        point_size: 2.0,
    };

    let mut euclid = Euclid::new();

    euclid.add_construction(Construction {
        shape: shapes::Shape::Segment(SegmentData {
            p1: Vec2::new(screen_width() / 2.0 - 10.0, screen_height() / 2.0),
            p2: Vec2::new(screen_width() / 2.0 + 10.0, screen_height() / 2.0),
        }),
        layer: 0,
        color: config.guide,
    });

    euclid.add_construction(Construction {
        shape: shapes::Shape::Segment(SegmentData {
            p1: Vec2::new(screen_width() / 2.0, screen_height() / 2.0 - 10.0),
            p2: Vec2::new(screen_width() / 2.0, screen_height() / 2.0 + 10.0),
        }),
        layer: 0,
        color: config.guide,
    });

    euclid.run(&config).await;
}
