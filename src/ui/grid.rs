pub fn new(id: &str) -> egui::Grid {
    egui::Grid::new(id).num_columns(1).striped(false)
}

pub fn text_row(ui: &mut egui::Ui, key: &str, text: &str) {
    add_row(ui, key, |ui| ui.monospace(text));
}

pub fn add_row<R>(ui: &mut egui::Ui, key: &str, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
    ui.horizontal(|ui| {
        ui.label(key);
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            add_contents,
        );
    });

    ui.end_row();
}

#[allow(dead_code)]
pub fn add_struct<T: serde::Serialize>(ui: &mut egui::Ui, data_struct: T) {
    let values = serde_json::to_value(data_struct).unwrap();

    if let serde_json::Value::Object(map) = values {
        for (k, v) in map.into_iter() {
            text_row(ui, &clean_key(&k), clean_value(&v.to_string()));
        }
    } else {
        panic!("struct did not serialize to object")
    }
}

fn clean_value(value: &str) -> &str {
    return value
        .strip_prefix('"')
        .unwrap_or(value)
        .strip_suffix('"')
        .unwrap_or(value);
}

fn clean_key(key: &str) -> String {
    let mut clean = String::new();

    for c in key.chars() {
        if c.is_uppercase() {
            clean.push(' ');
        }

        clean.push_str(&c.to_lowercase().to_string());
    }

    clean
}

pub fn separator(ui: &mut egui::Ui) {
    ui.separator();
    ui.end_row();
}
