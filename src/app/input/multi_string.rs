use crate::app::input::square_button;
use egui::TextStyle;

use egui::Color32;

pub(crate) trait StringValidate {
    fn validate(&self, value: &str) -> bool;
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub(crate) struct RegexValidator {}

impl StringValidate for RegexValidator {
    fn validate(&self, value: &str) -> bool {
        fancy_regex::Regex::new(value).is_ok()
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct StringArrayInput<Validator: StringValidate> {
    pub(crate) validator: Validator,
    pub(crate) name: String,

    pub(crate) strings: Vec<String>,
    #[serde(skip)]
    pub(crate) new_string: String,
}

impl<Validator> StringArrayInput<Validator>
where
    Validator: StringValidate,
{
    pub(crate) fn new(name: &str, validator: Validator) -> Self {
        Self {
            name: name.to_string(),
            strings: vec![],
            new_string: String::default(),
            validator,
        }
    }
    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.name);
        ui.group(|ui| {
            ui.vertical(|ui| {
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
        });
    }
}
