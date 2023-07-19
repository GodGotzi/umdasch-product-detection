use egui::RichText;
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::application::ProductDetectionApplication;

pub fn show_top_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top-panel").exact_height(20.0).show(ctx, |ui| {

        GridBuilder::new().new_row_align(Size::remainder(), egui::Align::Center)
            .cell(Size::remainder())
            .cell(Size::remainder())
            .show(ui, |mut grid| {
                grid.cell(|ui| {
                    let text = RichText::new("Umdasch - ProductDetectionApplication").strong();
                    ui.label(text);
                });

                grid.cell(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let text = RichText::new("@Elias Gottsbacher").strong();
                        ui.label(text);  
                    });
                });
            });
    });
}

pub fn show_main_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {

    egui::CentralPanel::default().show(ctx, |_ui| {
        show_training_panel(app, ctx, frame);
        show_terminal_panel(app, ctx, frame);
        show_detection_panel(app, ctx, frame);
    });

}

//fn calculate_expansion(ctx)

fn show_training_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x * 0.225;

    egui::SidePanel::left("training-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Hi")
            });
        });
}

fn show_terminal_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.y * 0.3;
    
    egui::TopBottomPanel::bottom("terminal-panel")
        .exact_height(exp)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Hi")
            });
        });
}

fn show_detection_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x  * 0.225;

    egui::SidePanel::right("detection-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Hi")
            });
        });
}

