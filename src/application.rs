
use crate::{
    gui, detection::{
        async_detector::AsyncDetector, DetectionContext, 
    }
};

use tokio::runtime::Runtime;

#[allow(unused)]
pub struct ProductDetectionApplication {
    pub _context: ApplicationContext,
    pub detection_context: DetectionContext,
    pub async_detector: AsyncDetector,
    pub tokio_runtime: Runtime
}

impl ProductDetectionApplication {

    pub fn new(rt: Runtime) -> Self {

        Self {
            _context: ApplicationContext::default(),
            detection_context: DetectionContext::default(),
            async_detector: AsyncDetector::new(),
            tokio_runtime: rt
        }
    
    }

}

impl ProductDetectionApplication {

    pub fn _reload(&mut self) {
        let handle = self.async_detector.load(&self.tokio_runtime);

        match handle {
            Some(_) => println!("Detection Started!"),
            None => println!("Detection already started/last Detection not finished"),
        }
    }

}

impl eframe::App for ProductDetectionApplication {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        if self.async_detector.is_finished() {
            if let Some((img, detections)) = self.async_detector.result() {
                self.detection_context.detections = Some(detections);
                self.detection_context.img = Some(img.0);   
            }
        }

        if let Some(rx) = self.detection_context.resize_rx.as_mut() {
            println!("yes");
            if let Ok(result) = rx.try_recv() {
                self.detection_context.resized_img = Some(result);
                self.detection_context.resize_rx = None;
                self.detection_context.resize_handle = None;
                self.detection_context.resizing = false;
            }
        }

        gui::show_top_panel(ctx);

        gui::show_main_panel(self, ctx, frame);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.async_detector.kill();
        self.detection_context.kill();
    }

}

#[derive(Default)]
pub struct ApplicationContext;