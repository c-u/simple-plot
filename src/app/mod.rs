use crate::app::regextractor_ui::RegextractorUi;

mod input;
mod regextractor_ui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    regextractor: RegextractorUi,
}

impl TemplateApp {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.regextractor.draw_side_panel(ui);
                Self::acknowledge(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.regextractor.draw_center_panel(ui);
        });
    }
}
