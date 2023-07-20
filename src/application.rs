
use crate::{
    gui, detection::{
        data::ImageDetections, 
        async_detector::SendableMat
    }
};

use tokio::sync::watch::*;

pub struct ProductDetectionApplication {
    _context: ApplicationContext,
    receiver_detections: Receiver<Option<ImageDetections>>,
    receiver_image: Receiver<Option<SendableMat>>,
    sender_reload: tokio::sync::mpsc::Sender<bool>,
    sender_enable: Sender<bool>
}

impl ProductDetectionApplication {

    pub fn new(
        receiver_detections: Receiver<Option<ImageDetections>>, 
        receiver_image: Receiver<Option<SendableMat>>, 
        sender_reload: tokio::sync::mpsc::Sender<bool>, 
        sender_enable: Sender<bool>) -> Self {

        Self {
            _context: ApplicationContext::default(),
            receiver_detections,
            receiver_image,
            sender_reload,
            sender_enable
        }
    
    }

}

impl ProductDetectionApplication {

    #[allow(unused_must_use)]
    fn _reload(&mut self) {
        self.sender_reload.send(false);
    } 

}

impl eframe::App for ProductDetectionApplication {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        match self.receiver_detections.has_changed() {
            Ok(changed) => {
                if changed {
                    println!("Changed Detections!");
                }
            },
            Err(err) => println!("Not Loaded {}", err),
        }

        match self.receiver_image.has_changed() {
            Ok(changed) => {
                if changed {
                    println!("Changed Image!");
                }
            },
            Err(err) => println!("Not Loaded {}", err),
        }

        gui::show_top_panel(ctx);

        gui::show_main_panel(self, ctx, frame);
    }

}


#[derive(Default)]
pub struct ApplicationContext;

#[derive(Default)]
pub struct DetectionContext {
    
}

