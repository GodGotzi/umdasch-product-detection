use egui::Ui;

use crate::application::ProductDetectionApplication;

pub fn show(app: &mut ProductDetectionApplication, ui: &mut Ui) {
    if ui.button("Detect").clicked() {
        app._reload();
    }
}