use std::f64::consts::PI;

use egui::{
    plot::{AxisBools, Legend, Line, Plot, PlotPoints},
    Color32,
};

use crate::file_reader;

struct PlotLine {
    name: String,
    data_points: Vec<[f64; 2]>,
    color: Color32,
    filled: bool,
    visible: bool,
}

impl Default for PlotLine {
    fn default() -> Self {
        Self::new(0.0, "default123", Color32::RED)
    }
}

impl PlotLine {
    fn new<S: Into<String>, C: Into<Color32>>(offset: f64, name: S, color: C) -> Self {
        Self {
            name: name.into(),
            visible: true,
            color: color.into(),
            data_points: example_sin_curve(offset).collect(),
            filled: false,
        }
    }
    fn get_line(&self) -> Line {
        let points: PlotPoints = self.data_points.clone().into();
        let mut line = Line::new(points).color(self.color).name(&self.name);
        if self.filled {
            line = line.fill(0.0);
        }
        line
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    lines: Vec<PlotLine>,
    #[serde(skip)]
    file: Option<file_reader::SelectedFile>,
    #[serde(skip)]
    file_selector: file_reader::AsyncFileSelector,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            lines: vec![
                Default::default(),
                PlotLine::new(0.25 * PI, "BLUE", Color32::BLUE),
                PlotLine::new(0.5 * PI, "GREEN", Color32::GREEN),
                PlotLine::new(0.75 * PI, "YELLOW", Color32::YELLOW),
                PlotLine::new(1.0 * PI, "from_rgb", Color32::from_rgb(184, 0, 200)),
            ],
            file: None,
            file_selector: file_reader::AsyncFileSelector::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn open_file(&mut self, ui: &mut egui::Ui) {
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
                ui.code(std::str::from_utf8(file.content()).unwrap_or_default());
            });
        }
    }

    fn draw_line_settings(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("line_grid").striped(true).show(ui, |ui| {
            for line in &mut self.lines {
                Self::draw_line_setting(ui, line);
                ui.end_row();
            }
        });
    }
    fn draw_line_setting(ui: &mut egui::Ui, line: &mut PlotLine) {
        // ui.group(|ui| {
        ui.heading(line.name.as_str());

        ui.add(egui::Checkbox::new(&mut line.visible, "visible"));
        ui.add(egui::Checkbox::new(&mut line.filled, "filled"));
        egui::color_picker::color_edit_button_srgba(
            ui,
            &mut line.color,
            egui::color_picker::Alpha::BlendOrAdditive,
        );
        // });
    }

    fn acknowledge(ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("based on ");
                ui.hyperlink_to(
                    "eframe_template",
                    "https://github.com/emilk/eframe_template",
                );
                ui.label(".");
            });
            egui::warn_if_debug_build(ui);
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.separator();
            self.draw_line_settings(ui);

            ui.horizontal(|ui| {
                ui.heading("Open File:");
                self.open_file(ui);
            });

            Self::acknowledge(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut zoom = AxisBools::new(true, true);
            // The central panel the region left after adding TopPanel's and SidePanel's
            if ui.input(|i| i.modifiers.shift) {
                zoom.x = false;
            }
            if ui.input(|i| i.modifiers.alt) {
                zoom.y = false;
            }

            Plot::new("my_plot")
                .allow_zoom(zoom)
                .legend(Legend::default())
                // .view_aspect(2.0)
                // .reset()
                .show(ui, |plot_ui| {
                    for line in &self.lines {
                        if line.visible {
                            plot_ui.line(line.get_line());
                        }
                    }
                });
        });
    }
}

fn example_sin_curve(offset: f64) -> impl Iterator<Item = [f64; 2]> {
    (0..10_000).map(move |i| {
        let x = i as f64 * 0.01;
        [x, (x - offset).sin()]
    })
}
