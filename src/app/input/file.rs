use super::file_reader::{self, SelectedFile};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct DataFile {
    #[serde(skip)]
    file: Option<file_reader::SelectedFile>,
    #[serde(skip)]
    file_selector: file_reader::AsyncFileSelector,
}

impl DataFile {
    pub(crate) fn new() -> Self {
        DataFile {
            file: None,
            file_selector: file_reader::AsyncFileSelector::new(),
        }
    }

    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("File");
        self.open_file(ui);
    }

    pub(crate) fn open_file(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open file…").clicked() {
            self.file.take();
            self.file_selector.select();
        }

        if self.file.is_none() {
            self.file = self.file_selector.get_file().into();
        }

        if let Some(file) = &self.file {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(file.file_name());
            });
        }
    }

    pub(crate) fn file(&self) -> Option<&SelectedFile> {
        self.file.as_ref()
    }
}
