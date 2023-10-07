use fancy_regex::Regex;

use crate::app::input::multi_string::{self, RegexValidator, StringArrayInput};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct Config {
    includes: StringArrayInput<RegexValidator>,
    excludes: StringArrayInput<RegexValidator>,
    use_regex_group: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_regex_group: false,
            includes: multi_string::StringArrayInput::new(
                "Includes",
                multi_string::RegexValidator {},
            ),
            excludes: multi_string::StringArrayInput::new(
                "Excludes",
                multi_string::RegexValidator {},
            ),
        }
    }
}

impl Config {
    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Config");
        self.includes.draw(ui);
        self.excludes.draw(ui);
    }

    pub(crate) fn get_includes(&self) -> impl Iterator<Item = Regex> + '_ {
        self.includes
            .strings
            .iter()
            .filter_map(|f| fancy_regex::Regex::new(f).ok())
    }
    pub(crate) fn get_excludes(&self) -> impl Iterator<Item = Regex> + '_ {
        self.excludes
            .strings
            .iter()
            .filter_map(|f| fancy_regex::Regex::new(f).ok())
    }
}
