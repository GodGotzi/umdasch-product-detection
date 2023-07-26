
use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::{
    gui, monitor::{
        async_detector::{AsyncDetector, SendableMat}, MonitorHandler, data::ImageDetections, 
    }, product::ProductServer, training::TrainingManager
};

use eframe::CreationContext;

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
    pub context: ApplicationContext,
    pub monitor_handler: MonitorHandler,
    pub async_detector: AsyncDetector,
    pub product_server: ProductServer,
    pub training_manager: Arc<Mutex<TrainingManager>>,
    tx_detections: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>,
    rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>,
    capture_enable: Arc<AtomicBool>
}

impl ProductDetectionApplication {

    pub fn new(detector: AsyncDetector, cc: &CreationContext, container: CaptureContainer, product_server: ProductServer) -> Self {
        let capture_enable = Arc::new(AtomicBool::new(true));
        
        tokio::spawn(AsyncDetector::capture_loop(cc.egui_ctx.clone(), container.tx_capture, capture_enable.clone()));

        let monitor = MonitorHandler::new(container.rx_capture.clone(), container.rx_detection);

        Self {
            context: ApplicationContext::default(),
            monitor_handler: monitor,
            async_detector: detector,
            product_server: product_server,
            training_manager: Arc::new(Mutex::new(TrainingManager::default())),
            tx_detections: container.tx_detection,
            rx_capture: container.rx_capture,
            capture_enable
        }
    }

}

impl ProductDetectionApplication {

    pub fn reload(&mut self) {
        self.async_detector.load(self.context.clone(), self.training_manager.clone(), self.rx_capture.clone(), self.tx_detections.clone());
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
        self.capture_enable.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    fn on_close_event(&mut self) -> bool {
        self.async_detector.kill();
        self.monitor_handler.kill();
        self.capture_enable.store(false, std::sync::atomic::Ordering::Relaxed);

        true
    }

}

#[derive(Clone)]
pub struct ApplicationContext {
    pub threshold: String,
    pub threshold_default: String,
    pub suppression: bool,
    pub filter_confidence: bool,
    pub stereo: bool,
}

impl Default for ApplicationContext {

    fn default() -> Self {
        Self { threshold: "0.1".into(), threshold_default: "0.1".into(), suppression: true, filter_confidence: true, stereo: false }        
    }

}

