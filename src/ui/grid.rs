use serde_yaml::Value;

pub fn new(id: &str) -> egui::Grid {
    egui::Grid::new(id).num_columns(1).striped(false)
}

pub fn add_text_row(ui: &mut egui::Ui, key: &str, text: &str) {
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

pub fn add_struct<T: serde::Serialize>(ui: &mut egui::Ui, data_struct: T) {
    let values = serde_yaml::to_value(data_struct).unwrap();

    if let serde_yaml::Value::Mapping(map) = values {
        for (k, v) in map.into_iter() {
            add_text_row(
                ui,
                &value_to_string(&k),
                &value_to_string(&v),
            );
        }
    } else {
        panic!("struct did not serialize to object")
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
        _ => todo!("handle other types"),
    }
}

pub fn separator(ui: &mut egui::Ui) {
    ui.separator();
    ui.end_row();
}
