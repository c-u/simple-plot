use std::{f64::consts::PI, io::BufRead};

use egui::{Color32, TextStyle};
use egui_plot::{AxisBools, Legend, Line, Plot, PlotPoints, Points};

use crate::file_reader;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] //
struct PlotLine {
    name: String,
    #[serde(skip)]
    data_points: Vec<[f64; 2]>,
    color: Color32,
    filled: bool,
    visible: bool,
    regex: String,
}

impl Default for PlotLine {
    fn default() -> Self {
        Self::new("", Color32::TRANSPARENT)
    }
}

impl PlotLine {
    fn new<S: Into<String>, C: Into<Color32>>(name: S, color: C) -> Self {
        Self {
            name: name.into(),
            visible: true,
            color: color.into(),
            data_points: vec![],
            filled: false,
            regex: String::default(),
        }
    }
    fn draw_line_setting(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // })
            // ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .char_limit(32)
                        .font(TextStyle::Monospace),
                );

                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.color,
                    egui::color_picker::Alpha::BlendOrAdditive,
                );
            });
            ui.horizontal(|ui| {
                if fancy_regex::Regex::new(&self.regex).is_err() {
                    ui.style_mut().visuals.extreme_bg_color = Color32::DARK_RED;
                }
                ui.label("Regex:");
                ui.add(egui::TextEdit::singleline(&mut self.regex).font(TextStyle::Monospace));
                ui.add(egui::Checkbox::new(&mut self.visible, "visible"));
                ui.add(egui::Checkbox::new(&mut self.filled, "filled"));
            });
        });
    }

    fn plot(&self, plot_ui: &mut egui_plot::PlotUi) {
        if self.visible {
            plot_ui.line(self.get_line());
            plot_ui.points(self.get_points());
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

    fn get_points(&self) -> Points {
        let points: PlotPoints = self.data_points.clone().into();
        Points::new(points)
            .color(self.color)
            .name(&self.name)
            .shape(egui_plot::MarkerShape::Cross)
            .radius(5.0)
    }

    fn update(&mut self, data: &regextractor::datatable::DataTable<f32>) {
        self.data_points.clear();
        if let Ok(d) = data.get_col_by_name_with_base(&self.name) {
            for point in d.filter(|f| !f.0.is_nan() && !f.1.is_nan()) {
                self.data_points.push([*point.0 as f64, *point.1 as f64]);
            }
        }
    }
    fn reset(&mut self) {
        self.data_points.clear();
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if
struct PlotLines {
    plot_lines: Vec<PlotLine>,
    selected_base_line: Option<usize>,
}

impl PlotLines {
    fn draw_line_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("PlotLines");
        let mut lines_to_remove: Vec<usize> = vec![];
        for (index, line) in &mut self.plot_lines.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if square_button(ui, '-').clicked() {
                    lines_to_remove.push(index);
                }

                let mut checked = false;
                if let Some(ix) = self.selected_base_line {
                    checked = ix == index;
                }
                if ui.checkbox(&mut checked, "").changed() {
                    self.selected_base_line = if checked { Some(index) } else { None };
                }

                line.draw_line_setting(ui);
                // Self::draw_line_setting(ui, line);
            });
            ui.separator();
        }

        ui.horizontal(|ui| {
            if square_button(ui, '+').clicked() {
                self.plot_lines.push(PlotLine::default())
            }
        });

        for index in lines_to_remove {
            self.plot_lines.remove(index);
        }
    }

    fn plot(&self, plot_ui: &mut egui_plot::PlotUi) {
        for line in self.plot_lines.iter() {
            line.plot(plot_ui)
        }
    }

    fn get_lines(&self) -> (Option<&PlotLine>, &Vec<PlotLine>) {
        if let Some(ix) = self.selected_base_line {
            (Some(&self.plot_lines[ix]), &self.plot_lines)
        } else {
            (None, &self.plot_lines)
        }
    }
    fn get_base_line_name(&self) -> Option<&str> {
        Some(&self.plot_lines[self.selected_base_line?].name)
    }

    fn update(&mut self, data: &regextractor::datatable::DataTable<f32>) {
        for (ix, line) in self.plot_lines.iter_mut().enumerate() {
            if self.selected_base_line.is_some_and(|s| s == ix) {
                line.reset();
                continue;
            }
            line.update(data);
        }
    }
}

fn square_button(ui: &mut egui::Ui, symbol: char) -> egui::Response {
    ui.add_sized([20.0, 20.0], egui::Button::new(symbol.to_string()))
}

trait StringValidate {
    fn validate(&self, value: &str) -> bool;
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct RegexValidator {}

impl StringValidate for RegexValidator {
    fn validate(&self, value: &str) -> bool {
        fancy_regex::Regex::new(value).is_ok()
    }
}
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct StringArrayInput<Validator: StringValidate> {
    validator: Validator,
    name: String,

    strings: Vec<String>,
    #[serde(skip)]
    new_string: String,
}

impl<Validator> StringArrayInput<Validator>
where
    Validator: StringValidate,
{
    fn new(name: &str, validator: Validator) -> Self {
        Self {
            name: name.to_string(),
            strings: vec![],
            new_string: String::default(),
            validator,
        }
    }
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading(&self.name);

            let mut lines_to_remove: Vec<usize> = vec![];
            for (i, s) in self.strings.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if square_button(ui, '-').clicked() {
                        lines_to_remove.push(i);
                    }
                    ui.monospace(s);
                });
                ui.end_row();
            }
            for index in lines_to_remove {
                self.strings.remove(index);
            }

            ui.horizontal(|ui| {
                if square_button(ui, '+').clicked()
                    && self.validator.validate(&self.new_string)
                    && !self.new_string.is_empty()
                {
                    self.strings.push(self.new_string.clone());
                    self.new_string = String::default();
                }
                if !self.validator.validate(&self.new_string) {
                    ui.style_mut().visuals.extreme_bg_color = Color32::DARK_RED;
                }
                ui.add(
                    egui::TextEdit::singleline(&mut self.new_string)
                        .font(TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });
        });
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct DataFile {
    includes: StringArrayInput<RegexValidator>,
    excludes: StringArrayInput<RegexValidator>,
    use_regex_group: bool,

    #[serde(skip)]
    file: Option<file_reader::SelectedFile>,
    #[serde(skip)]
    file_selector: file_reader::AsyncFileSelector,
}

impl DataFile {
    fn new() -> Self {
        DataFile {
            use_regex_group: false,
            includes: StringArrayInput::new("Includes", RegexValidator {}),
            excludes: StringArrayInput::new("Excludes", RegexValidator {}),
            file: None,
            file_selector: file_reader::AsyncFileSelector::new(),
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Open File:");
        self.open_file(ui);

        ui.separator();
        self.includes.draw(ui);
        ui.separator();
        self.excludes.draw(ui);
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
            });
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    lines: PlotLines,
    data_file: DataFile,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            lines: PlotLines {
                plot_lines: vec![],
                selected_base_line: None,
            },
            data_file: DataFile::new(),
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
            println!("Using saved context");
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.lines.draw_line_settings(ui);
                ui.separator();

                self.data_file.draw(ui);
                ui.separator();

                if ui.button("Update").clicked() {
                    if let Some(data) = &self.data_file.file {
                        let data_regex = self
                            .lines
                            .plot_lines
                            .iter()
                            .filter_map(|f| {
                                if let Ok(regex) = fancy_regex::Regex::new(&f.regex) {
                                    Some(regextractor::NamedRegex {
                                        name: f.name.clone(),
                                        regex,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let included_lines = self
                            .data_file
                            .includes
                            .strings
                            .iter()
                            .filter_map(|f| fancy_regex::Regex::new(f).ok())
                            .collect();
                        let excluded_lines = self
                            .data_file
                            .excludes
                            .strings
                            .iter()
                            .filter_map(|f| fancy_regex::Regex::new(f).ok())
                            .collect();

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
                .show(ui, |plot_ui| self.lines.plot(plot_ui));
        });
    }
}
