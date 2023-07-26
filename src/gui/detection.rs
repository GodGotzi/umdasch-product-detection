use std::collections::HashMap;

use egui::{Ui, Vec2, Layout};

use crate::{application::ProductDetectionApplication, product::Product};

pub fn show(app: &mut ProductDetectionApplication, ui: &mut Ui, frame: &mut eframe::Frame) {
    ui.heading("Possible Products");
    ui.separator();

    let exp = frame.info().window_info.size.y * 0.35;

    egui::TopBottomPanel::bottom("bottom-detection-panel")
        .exact_height(exp)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {

                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        let detect_button = egui::Button::new("Detect")
                            .min_size(Vec2::new(ui.available_width()*0.7, 30.0));

                        if ui.add(detect_button).clicked() {
                            app.reload();
                        }
                    });
                    ui.separator();

                    ui.add_space(5.0);

                    ui.heading("Detection Configuration");
                    ui.add_space(25.0);

                    ui.allocate_ui(Vec2::new(ui.available_width(), ui.available_height()), |ui| {
                        egui::ScrollArea::both()
                        .max_height(ui.available_height())
                        .show(ui, |ui| {

                            ui.horizontal(|ui| {
                                ui.label("Threshhold (float): ");
                                ui.text_edit_singleline(&mut app.context.threshold);
                            });

                            //ui.checkbox(&mut app.context.filter_confidence, "Confidence Filter ");
                            ui.checkbox(&mut app.context.suppression, "Suppression ");
                            ui.checkbox(&mut app.context.stereo, "Stereo ");
                            //let mut buffer = String::from("Hi whats up");
                        });
                    });
                });
            });
        });

    egui::CentralPanel::default()
        .show_inside(ui, |ui| {
            if let Some(detections) = app.monitor_handler.detection.get_detections() {

                egui::ScrollArea::both()
                .show(ui, |ui| {
                    
                    for detection in detections.detections.iter() {
                        ui.allocate_ui(Vec2::new(ui.available_width(), 40.0), |ui| {
                            ui.vertical(|ui| {
                                ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                    let product = match (detection.class_index as usize) < app.product_server.len() {
                                        true => app.product_server.get_by_id(detection.class_index as usize).clone(),
                                        false => Product { path: None, number: "Unkown".into(), name: "Unkown".into(), class_id: 0, surface_color: HashMap::new() },
                                    };
                                    
                                    ui.vertical(|ui| {
                                        ui.heading(product.name.as_str());
        
                                        ui.label(format!("Confidence: {}", detection.confidence));
                                        ui.label(format!("Class Index: {}", detection.class_index));
                                        ui.label(format!("Boundaries: x: {}, y: {}, width: {}, height: {}", detection.x, detection.y, detection.width, detection.height));
            
                                        ui.separator();
                                    });
                                });
                            });
                        });
                    }
                });
            }
        });
}