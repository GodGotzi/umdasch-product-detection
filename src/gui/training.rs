use egui::{Ui, Layout, Vec2};

use crate::application::ProductDetectionApplication;

pub fn show(app: &mut ProductDetectionApplication, ui: &mut Ui, frame: &mut eframe::Frame) {
    ui.heading("Data");
    ui.separator();

    let exp = frame.info().window_info.size.y * 0.35;

    egui::TopBottomPanel::bottom("bottom-training-panel")
        .exact_height(exp)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {

                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    
                    ui.separator();

                    ui.add_space(5.0);

                    ui.heading("Training Setup");
                    ui.add_space(25.0);

                    ui.allocate_ui(Vec2::new(ui.available_width(), ui.available_height()), |ui| {
                        egui::ScrollArea::both()
                        .max_height(ui.available_height())
                        .show(ui, |_ui| {

                        });
                    });

                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {

                        let generate_samples_button = egui::Button::new("Generate Samples")
                            .min_size(Vec2::new(ui.available_width()*0.7, 30.0));

                        ui.add(generate_samples_button);

                        ui.add_space(5.0);
                        let train_button = egui::Button::new("Train")
                            .min_size(Vec2::new(ui.available_width()*0.7, 30.0));

                        if ui.add(train_button).clicked() {
                            app.reload();
                        }

                    });
                });
            });
        });

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