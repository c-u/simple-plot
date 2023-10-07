pub(crate) mod file;
pub(crate) mod multi_string;

mod file_reader;

pub(crate) fn square_button(ui: &mut egui::Ui, symbol: char) -> egui::Response {
    ui.add_sized([20.0, 20.0], egui::Button::new(symbol.to_string()))
}
