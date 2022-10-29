mod euclid;
mod shape;
mod tool;
mod utils;

use euclid::*;
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

#[macroquad::main(window_conf)]
async fn main() {
    let gruvbox = ColorPalette {
        black: Color::from_rgba(10, 10, 10, 255),
        white: Color::from_rgba(235, 219, 178, 255),
        gray: Color::from_rgba(168, 154, 132, 200),
        red: Color::from_rgba(204, 36, 29, 255),
        green: Color::from_rgba(152, 151, 26, 255),
        yellow: Color::from_rgba(215, 153, 33, 255),
        blue: Color::from_rgba(69, 133, 136, 255),
        purple: Color::from_rgba(177, 98, 134, 255),
    };

    let roboto = load_ttf_font("./assets/fonts/RobotoMono.ttf")
        .await
        .expect("failed to load font");

    Euclid::new(gruvbox, roboto).run().await;
}
