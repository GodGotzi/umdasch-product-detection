use std::sync::Mutex;

use crate::gui;

#[derive(Default)]
pub struct ProductDetectionApplication {
    _context: ApplicationContext,
    _detection_context: Mutex<DetectionContext>
}

impl eframe::App for ProductDetectionApplication {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        gui::show_top_panel(ctx);

        gui::show_main_panel(self, ctx, frame);
    }

}


#[derive(Default)]
pub struct ApplicationContext;

#[derive(Default)]
pub struct DetectionContext {

}

