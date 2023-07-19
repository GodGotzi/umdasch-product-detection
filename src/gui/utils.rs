use egui::Ui;

pub fn with_heading_panel(text: &str, ctx: &egui::Context) {
    egui::TopBottomPanel::top(format!("heading-panel-{}", text))
        .exact_height(20.0)
        .show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.label(text);
                });
        });
}

pub fn with_heading(text: &str, ui: &mut Ui) {

    egui::TopBottomPanel::top(format!("heading-panel-{}", text))
    .exact_height(20.0)
    .show_inside(ui, |ui| {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.label(text);
            });
    });
}