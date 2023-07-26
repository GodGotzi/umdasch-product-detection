pub mod detection;
pub mod terminal;
pub mod training;
pub mod utils;

use egui::{RichText, Vec2};
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
    show_training_panel(app, ctx, frame);

    utils::with_heading_panel("Detection", ctx);
    show_detection_panel(app, ctx, frame);
    show_terminal_panel(app, ctx, frame);

    egui::CentralPanel::default().show(ctx, |ui| {
        let size = match app.monitor_handler.detection.is_bundle_arrived() {
            true => opencv::core::Size::new((ui.available_width() as f32 / 2.0) as i32, ui.available_height() as i32),
            false => opencv::core::Size::new(ui.available_width() as i32, ui.available_height() as i32),
        };

        app.monitor_handler.capture
            .resize(size.clone(), app.training_manager.clone());

        app.monitor_handler.detection
            .resize(size.clone(), &app.product_server, app.training_manager.clone());

        if app.monitor_handler.capture.get_image().is_some() && app.monitor_handler.detection.get_image().is_none() {
            let img = app.monitor_handler.capture.get_image().unwrap();

            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.image(img.texture_id(ctx), Vec2::new(img.width() as f32, img.height() as f32));
            });
        } else if app.monitor_handler.capture.get_image().is_some() && app.monitor_handler.detection.get_image().is_some() {
            GridBuilder::new()
                .new_row_align(Size::remainder(), egui::Align::Center)
                .cell(Size::remainder())
                .cell(Size::remainder())
                .show(ui, |mut grid| {
                    grid.cell(|ui| {
                        let img = app.monitor_handler.capture.get_image().unwrap();

                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                            ui.image(img.texture_id(ctx), Vec2::new(img.width() as f32, img.height() as f32));
                        });
                    });
                    grid.cell(|ui| {
                        let img = app.monitor_handler.detection.get_image().unwrap();

                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                            ui.image(img.texture_id(ctx), Vec2::new(img.width() as f32, img.height() as f32));
                        });
                    });
                });
        }

    });

}

//fn calculate_expansion(ctx)

fn show_training_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x * 0.2;

    egui::SidePanel::left("training-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            utils::with_heading("Training", ui);

            training::show(app, ui, frame);
        });
}

fn show_terminal_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.y * 0.35;
    
    egui::TopBottomPanel::bottom("terminal-panel")
        .exact_height(exp)
        .resizable(false)
        .show(ctx, |ui| {
            terminal::show(app, ui, frame);

        });
}

fn show_detection_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x  * 0.2;

    egui::SidePanel::right("detection-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            detection::show(app, ui, frame);
        });
}

