use std::sync::{Arc, atomic::AtomicBool, Mutex};

use opencv::{
    prelude::*, 
    videoio::{
        VideoCapture, 
        VideoCaptureProperties
    }, 
    imgproc::INTER_LINEAR,
};

use tokio::task::JoinHandle;

use crate::{monitor::{model::DNNModel, data::ImageDetections}, application::ApplicationContext, training::TrainingManager};

pub struct AsyncDetector {
    handle: Option<JoinHandle<()>>,
    model: DNNModel
}

unsafe impl Sync for AsyncDetector {}

#[derive(Clone, Debug)]
pub struct SendableMat(pub Mat);

unsafe impl Send for SendableMat {}
unsafe impl Sync for SendableMat {}

impl AsyncDetector {

    pub fn new() -> Self {
        Self { 
            handle: None, 
            model: DNNModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap(),
        }
    }

    pub async fn capture_loop(ctx: egui::Context, tx: tokio::sync::watch::Sender<Option<((), SendableMat)>>, capture_enable: Arc<AtomicBool>) {
        while capture_enable.load(std::sync::atomic::Ordering::Relaxed) {
            let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32, 3840.0).unwrap();
            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32, 2080.0).unwrap();
    
            let opened = cam.is_opened().unwrap();
    
            if !opened {
                println!("Couldn't find Webcamera!");
            }
    
            let mut raw_frame = Mat::default();
    
                //println!("{}", AsyncDetector::empty_or_value(&mut rx_reload, true));
            let _result = cam.read(&mut raw_frame).unwrap();
    
            let cropped_mat = raw_frame.apply(
                opencv::core::Range::new(0, raw_frame.rows()).unwrap(), 
                opencv::core::Range::new((raw_frame.cols() - raw_frame.rows()) / 2, raw_frame.cols() - ((raw_frame.cols() - raw_frame.rows()) / 2)).unwrap()).unwrap();
    
            let mut frame = Mat::default();
    
            opencv::imgproc::resize(&cropped_mat, &mut frame, opencv::core::Size::new(2048, 2048), 0.0, 0.0, INTER_LINEAR).unwrap();

            ctx.request_repaint();

            let _ = tx.send(Some(((), SendableMat(frame.clone()))));
        }
    }

    async fn load_async(context: ApplicationContext, training_manager: Arc<Mutex<TrainingManager>>, tx: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>, mut rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>, mut model: DNNModel) {
        if let Some(matrix) = rx_capture.borrow_and_update().as_ref() {
            training_manager.lock().unwrap().print("Starting Detection!");

            let threshold: f32 = match context.threshold.parse() {
                Ok(val) => val,
                Err(_) => context.threshold_default.parse().unwrap(),
            };

            training_manager.lock().unwrap().print("Detecting...");
            let detections = model.detect(&matrix.1.0, threshold, context.suppression, context.filter_confidence, training_manager.clone()).unwrap();
            
            training_manager.lock().unwrap().print("Sending Data...");
            tx.send(Some((detections, matrix.1.clone()))).unwrap();
            training_manager.lock().unwrap().print("Data send");
        }
    }

    pub fn load(&mut self, context: ApplicationContext, training_manager: Arc<Mutex<TrainingManager>>, rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>, tx_detections: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>) -> Option<&JoinHandle<()>> {
        if let Some(handle) = &self.handle {
            if !handle.is_finished() {
                return None;
            }
        }

        self.handle = Some(tokio::spawn(AsyncDetector::load_async(context, training_manager, tx_detections, rx_capture, self.model.clone())));
        self.handle.as_ref()
    }

    pub fn _is_finished(&self) -> bool {
        if self.handle.is_none() {
            return true;
        }

        self.handle.as_ref().unwrap().is_finished()
    }

    pub fn kill(&mut self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }   
    }

}

