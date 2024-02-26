use std::ffi::OsString;

pub struct SaveWindow<'a> {
    open: &'a mut bool,
    file_name: &'a mut Option<String>,
}

impl<'a> SaveWindow<'a> {
    pub fn new(open: &'a mut bool, file_name: &'a mut Option<String>) -> Self {
        SaveWindow { open, file_name }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        if !*self.open {
            return false;
        }

        let mut closed = false;

        let mut file_name = self.file_name.clone().unwrap_or("unnamed".to_string());

        egui::Window::new("save file")
            .open(&mut self.open)
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("file name");
                    ui.add_space(16.0);
                    ui.text_edit_singleline(&mut file_name);
                });

                closed = ui.button("save").clicked();
            });

        *self.file_name = Some(file_name);

        closed
    }
}

pub struct OpenWindow<'a> {
    open: &'a mut bool,
    file_name: &'a mut Option<String>,
}

impl<'a> OpenWindow<'a> {
    pub fn new(open: &'a mut bool, file_name: &'a mut Option<String>) -> Self {
        OpenWindow { open, file_name }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        if !*self.open {
            return false;
        }

        let mut closed = false;

        let file_names = std::fs::read_dir("saves")
            .expect("could not read saves folder")
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("could not read saves folder");

        let mut file_name = if self.file_name.is_none()
            || !file_names
                .iter()
                .map(|f| get_file_name(f))
                .collect::<Vec<String>>()
                .contains(&self.file_name.clone().unwrap_or("".to_string()))
        {
            file_names
                .first()
                .map(|f| get_file_name(f))
                .unwrap_or("no saves".to_string())
        } else {
            self.file_name.clone().unwrap()
        };

        egui::Window::new("open file")
            .open(&mut self.open)
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("file name");
                    ui.add_space(16.0);

                    egui::ComboBox::from_id_source("file-dropdown")
                        .selected_text(file_name.clone())
                        .show_ui(ui, |ui| {
                            for file in file_names.iter() {
                                ui.selectable_value(
                                    &mut file_name,
                                    get_file_name(file),
                                    get_file_name(file),
                                );
                            }
                        });
                });

                closed = ui.button("open").clicked();
            });

        *self.file_name = Some(file_name);

        closed
    }
}

fn get_file_name(file: &OsString) -> String {
    file.to_string_lossy()
        .strip_suffix(".yml")
        .unwrap()
        .to_string()
}
