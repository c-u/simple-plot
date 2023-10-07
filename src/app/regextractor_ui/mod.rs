use egui_plot::{AxisBools, Legend, Plot};

use self::{config::Config, plotlines::PlotLines};

use super::input::file::DataFile;

pub(crate) mod config;
pub(crate) mod plotlines;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct RegextractorUi {
    lines: PlotLines,
    data_file: DataFile,
    config: Config,
}
impl RegextractorUi {
    pub(crate) fn draw_side_panel(&mut self, ui: &mut egui::Ui) {
        self.data_file.draw(ui);
        ui.separator();
        self.config.draw(ui);
        ui.separator();

        self.lines.draw_line_settings(ui);

        ui.separator();
        if ui
            .button(egui::RichText::new("Update").size(24.0))
            .clicked()
        {
            self.update_data_table();
        }
    }

    pub(crate) fn draw_center_panel(&mut self, ui: &mut egui::Ui) {
        let mut zoom = AxisBools::new(true, true);

        if ui.input(|i| i.modifiers.shift) {
            zoom.x = false;
        }
        if ui.input(|i| i.modifiers.alt) {
            zoom.y = false;
        }

        Plot::new("my_plot")
            .allow_zoom(zoom)
            .legend(Legend::default())
            .show(ui, |plot_ui| self.lines.plot(plot_ui));
    }
    pub(crate) fn update_data_table(&mut self) {
        if let Some(data) = &self.data_file.file() {
            let data_regex = self.lines.get_regular_expressions().collect();
            let included_lines = self.config.get_includes().collect();
            let excluded_lines = self.config.get_excludes().collect();

            if let Ok(data_table) = regextractor::extract_data(
                data.content(),
                data_regex,
                included_lines,
                excluded_lines,
                self.lines.get_base_line_name(),
                false,
            ) {
                self.lines.update(&data_table);
            }
        }
    }
}
