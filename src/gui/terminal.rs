use egui::{Ui, TextEdit};
use egui_grid::GridBuilder;
use egui_extras::Size;

use crate::application::ProductDetectionApplication;

pub fn show(app: &mut ProductDetectionApplication, ui: &mut Ui, _frame: &mut eframe::Frame) {

    egui::CentralPanel::default()
        .show_inside(ui, |ui| {
            GridBuilder::new()
                .new_row_align(Size::remainder(), egui::Align::Center)
                .cell(Size::remainder())
                .cell(Size::relative(0.6))
                .show(ui, |mut grid| {
                    grid.cell(|ui| {
                        egui::ScrollArea::both()
                            .id_source("training-terminal")
                            .show(ui, |ui| {
                                TextEdit::multiline(&mut app.training_manager.lock().unwrap().output).code_editor()
                                .min_size(egui::Vec2::new(ui.available_width(), ui.available_height()))
                                .show(ui);
                            });
                    });

                    grid.cell(|ui| {
                        egui::ScrollArea::both()
                            .id_source("detection-terminal")
                            .show(ui, |ui| {
                                TextEdit::multiline(&mut app.training_manager.lock().unwrap().output).code_editor().clip_text(false)
                                    .interactive(false)
                                    .min_size(egui::Vec2::new(ui.available_width(), ui.available_height()))
                                    .show(ui);
                            });
                    });
                });


        });

}