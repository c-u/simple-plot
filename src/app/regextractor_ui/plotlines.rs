use crate::app::input;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] //
pub(crate) struct PlotLine {
    pub(crate) name: String,
    #[serde(skip)]
    pub(crate) data_points: Vec<[f64; 2]>,
    pub(crate) color: egui::Color32,
    pub(crate) filled: bool,
    pub(crate) visible: bool,
    pub(crate) regex: String,
}

impl Default for PlotLine {
    fn default() -> Self {
        Self::new("", egui::Color32::WHITE)
    }
}

impl PlotLine {
    pub(crate) fn new<S: Into<String>, C: Into<egui::Color32>>(name: S, color: C) -> Self {
        Self {
            name: name.into(),
            visible: true,
            color: color.into(),
            data_points: vec![],
            filled: false,
            regex: String::default(),
        }
    }
    pub(crate) fn draw_line_setting(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // })
            // ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.spacing();
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .desired_width(200.0)
                        .char_limit(32)
                        .font(egui::TextStyle::Monospace),
                );

                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.color,
                    egui::color_picker::Alpha::BlendOrAdditive,
                );
            });
            ui.horizontal(|ui| {
                if fancy_regex::Regex::new(&self.regex).is_err() {
                    ui.style_mut().visuals.extreme_bg_color = egui::Color32::DARK_RED;
                }
                ui.label("Regex:");
                ui.spacing();
                ui.add(
                    egui::TextEdit::singleline(&mut self.regex)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(200.0),
                );
                ui.add(egui::Checkbox::new(&mut self.visible, "visible"));
                ui.add(egui::Checkbox::new(&mut self.filled, "filled"));
            });
        });
    }

    pub(crate) fn plot(&self, plot_ui: &mut egui_plot::PlotUi) {
        if self.visible {
            plot_ui.line(self.get_line());
            plot_ui.points(self.get_points());
        }
    }

    pub(crate) fn get_line(&self) -> egui_plot::Line {
        let points: egui_plot::PlotPoints = self.data_points.clone().into();
        let mut line = egui_plot::Line::new(points)
            .color(self.color)
            .name(&self.name);
        if self.filled {
            line = line.fill(0.0);
        }
        line
    }

    pub(crate) fn get_points(&self) -> egui_plot::Points {
        let points: egui_plot::PlotPoints = self.data_points.clone().into();
        egui_plot::Points::new(points)
            .color(self.color)
            .name(&self.name)
            .shape(egui_plot::MarkerShape::Cross)
            .radius(5.0)
    }

    pub(crate) fn update(&mut self, data: &regextractor::datatable::DataTable<f32>) {
        self.data_points.clear();
        if let Ok(d) = data.get_col_by_name_with_base(&self.name) {
            for point in d.filter(|f| !f.0.is_nan() && !f.1.is_nan()) {
                self.data_points.push([*point.0 as f64, *point.1 as f64]);
            }
        }
    }
    pub(crate) fn reset(&mut self) {
        self.data_points.clear();
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct PlotLines {
    pub(crate) plot_lines: Vec<PlotLine>,
    pub(crate) selected_base_line: Option<usize>,
}

impl PlotLines {
    pub(crate) fn draw_line_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("PlotLines");
        let mut lines_to_remove: Vec<usize> = vec![];
        for (index, line) in &mut self.plot_lines.iter_mut().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if input::square_button(ui, '-').clicked() {
                            lines_to_remove.push(index);
                        }

                        let mut checked = false;
                        if let Some(ix) = self.selected_base_line {
                            checked = ix == index;
                        }
                        if ui.checkbox(&mut checked, "onX").changed() {
                            self.selected_base_line = if checked { Some(index) } else { None };
                        }
                    });

                    line.draw_line_setting(ui);
                });
            });
        }

        ui.horizontal(|ui| {
            if input::square_button(ui, '+').clicked() {
                self.plot_lines.push(PlotLine::default())
            }
        });

        for index in lines_to_remove {
            self.plot_lines.remove(index);
        }
    }

    pub(crate) fn plot(&self, plot_ui: &mut egui_plot::PlotUi) {
        for line in self.plot_lines.iter() {
            line.plot(plot_ui)
        }
    }

    pub(crate) fn get_base_line_name(&self) -> Option<&str> {
        Some(&self.plot_lines[self.selected_base_line?].name)
    }

    pub(crate) fn update(&mut self, data: &regextractor::datatable::DataTable<f32>) {
        for (ix, line) in self.plot_lines.iter_mut().enumerate() {
            if self.selected_base_line.is_some_and(|s| s == ix) {
                line.reset();
                continue;
            }
            line.update(data);
        }
    }

    pub(crate) fn get_regular_expressions(
        &self,
    ) -> impl Iterator<Item = regextractor::NamedRegex> + '_ {
        self.plot_lines
            .iter()
            .filter_map(|f| regextractor::NamedRegex::new_from_string(&f.name, &f.regex))
    }
}
