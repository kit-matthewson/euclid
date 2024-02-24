use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml;

// Maybe a macro would make this suck less

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub colors: HashMap<String, egui::Color32>,

    pub tool_colors: Vec<egui::Color32>,

    pub background_color: egui::Color32,
    pub faint_background_color: egui::Color32,
    pub extreme_background_color: egui::Color32,

    pub grid_color: egui::Color32,
    pub intersection_color: egui::Color32,
    pub point_color: egui::Color32,
    pub text_color: egui::Color32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    colors: HashMap<String, String>,

    tool_colors: Vec<String>,
    background_color: String,
    faint_background_color: String,
    grid_color: String,
    intersection_color: String,
    point_color: String,
    text_color: String,
}

impl EngineConfig {
    pub fn read(file_name: &str) -> Self {
        let raw = std::fs::read_to_string(file_name).unwrap();
        let config: Config = serde_yaml::from_str(&raw).unwrap();

        Self {
            colors: config
                .colors
                .iter()
                .map(|(name, hex)| (name.clone(), Self::hex_str_to_rgba(hex)))
                .collect(),

            tool_colors: config
                .tool_colors
                .iter()
                .map(|color| {
                    Self::hex_str_to_rgba(config.colors.get(color).expect("tool color not found"))
                })
                .collect(),

            background_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.background_color)
                    .expect("background color not found"),
            ),
            faint_background_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.faint_background_color)
                    .expect("faint background color not found"),
            ),
            extreme_background_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.faint_background_color)
                    .expect("extreme background color not found"),
            ),

            grid_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.grid_color)
                    .expect("grid color not found"),
            ),
            intersection_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.intersection_color)
                    .expect("intersection color not found"),
            ),
            point_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.point_color)
                    .expect("point color not found"),
            ),
            text_color: Self::hex_str_to_rgba(
                config
                    .colors
                    .get(&config.text_color)
                    .expect("text color not found"),
            ),
        }
    }

    fn hex_str_to_rgba(hex: &str) -> egui::Color32 {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

        egui::Color32::from_rgba_premultiplied(r, g, b, 255)
    }

    pub fn get_name(&self, color: &egui::Color32) -> Option<String> {
        self.colors
            .iter()
            .find_map(|(name, c)| if c == color { Some(name.clone()) } else { None })
    }
}
