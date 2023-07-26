use egui::{Ui, Layout, Vec2};

use crate::application::ProductDetectionApplication;

pub fn show(app: &mut ProductDetectionApplication, ui: &mut Ui) {
    egui::CentralPanel::default()
    .show_inside(ui, |ui| {

            egui::ScrollArea::both()
            .show(ui, |ui| {
                
                for product in app.product_server.products().iter() {
                    ui.allocate_ui(Vec2::new(ui.available_width(), 40.0), |ui| {
                        ui.vertical(|ui| {
                            ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                
                                ui.vertical(|ui| {
                                    ui.heading(product.name.as_str());
                                    
                                    ui.label(format!("Class Index: {}", product.class_id));

                                    if let Some(path) = product.path.clone() {
                                        ui.label(format!("Path: {}", path));
                                    }
                                    
                                    ui.label(format!("Surface: r={} g={} b={}", product.surface_color.get("red").unwrap(), product.surface_color.get("green").unwrap(), product.surface_color.get("blue").unwrap()));
        
                                    ui.separator();
                                });
                            });
                        });
                    });
                }
            });
    });
}