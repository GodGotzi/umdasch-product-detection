
use crate::{
    gui, monitor::{
        async_detector::{AsyncDetector, SendableMat}, MonitorHandler, data::ImageDetections, 
    }
};

use eframe::CreationContext;
use tokio::runtime::Runtime;

pub struct CaptureContainer {
    tx_capture: tokio::sync::watch::Sender<Option<((), SendableMat)>>, 
    rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>,
    tx_detection: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>, 
    rx_detection: tokio::sync::mpsc::UnboundedReceiver<Option<(ImageDetections, SendableMat)>>
}

impl CaptureContainer {


    pub fn new(
        tx_capture: tokio::sync::watch::Sender<Option<((), SendableMat)>>, 
        rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>,
        tx_detection: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>, 
        rx_detection: tokio::sync::mpsc::UnboundedReceiver<Option<(ImageDetections, SendableMat)>>) -> Self {

        Self {
            tx_capture,
            rx_capture,
            tx_detection,
            rx_detection,
        }
    }

}

#[allow(unused)]
pub struct ProductDetectionApplication {
    pub _context: ApplicationContext,
    pub monitor_handler: MonitorHandler,
    pub async_detector: AsyncDetector,
    pub tokio_runtime: Runtime,
    tx_detections: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>,
    rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>
}

impl ProductDetectionApplication {

    pub fn new(rt: Runtime, detector: AsyncDetector, cc: &CreationContext, container: CaptureContainer) -> Self {
        rt.spawn(AsyncDetector::capture_loop(cc.egui_ctx.clone(), container.tx_capture));

        let monitor = MonitorHandler::new(container.rx_capture.clone(), container.rx_detection);
        
        Self {
            _context: ApplicationContext::default(),
            monitor_handler: monitor,
            async_detector: detector,
            tokio_runtime: rt,
            tx_detections: container.tx_detection,
            rx_capture: container.rx_capture
        }
    
    }

}

impl ProductDetectionApplication {

    pub fn reload(&mut self) {
        let handle = self.async_detector.load(&self.tokio_runtime, self.rx_capture.clone(), self.tx_detections.clone());

        match handle {
            Some(_) => println!("Detection Started!"),
            None => println!("Detection already started/last Detection not finished"),
        }
    }

}

impl eframe::App for ProductDetectionApplication {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        self.monitor_handler.capture.check();
        self.monitor_handler.detection.check();

        gui::show_top_panel(ctx);
        gui::show_main_panel(self, ctx, frame);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.async_detector.kill();
        self.monitor_handler.kill();
    }

    fn on_close_event(&mut self) -> bool {
        self.async_detector.kill();
        self.monitor_handler.kill();

        true
    }

}

#[derive(Default)]
pub struct ApplicationContext;