use image::ImageBuffer;
use opencv::{
    prelude::*, 
    videoio::{
        VideoCapture, 
        VideoCaptureProperties
    }, 
    imgproc::INTER_LINEAR,
};

use tokio::task::JoinHandle;

use crate::monitor::{model::DNNModel, data::ImageDetections};

pub struct AsyncDetector {
    handle: Option<JoinHandle<()>>,
    model: DNNModel
}

unsafe impl Sync for AsyncDetector {}

#[derive(Clone, Debug)]
pub struct SendableMat(pub Mat);

unsafe impl Send for SendableMat {}
unsafe impl Sync for SendableMat {}

#[derive(Clone, Debug)]
pub struct SendableImage(pub ImageBuffer<image::Rgba<u8>, Vec<u8>>);

unsafe impl Send for SendableImage {}
unsafe impl Sync for SendableImage {}

impl AsyncDetector {

    pub fn new() -> Self {
        Self { 
            handle: None, 
            model: DNNModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap(),
        }
    }

    pub async fn capture_loop(ctx: egui::Context, tx: tokio::sync::watch::Sender<Option<((), SendableMat)>>) {
        loop {
            let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32, 3840.0).unwrap();
            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32, 2080.0).unwrap();
    
            let opened = cam.is_opened().unwrap();
    
            if !opened {
                panic!("Couldn't find Webcamera!");
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
            tx.send(Some(((), SendableMat(frame.clone())))).unwrap();
        }
    }

    async fn load_async(tx: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>, mut rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>, mut model: DNNModel) {
        if let Some(matrix) = rx_capture.borrow_and_update().as_ref() {
            let detections = model.detect(&matrix.1.0, 0.6, 0.45).unwrap();
            //println!("Computed until render");
            tx.send(Some((detections, matrix.1.clone()))).unwrap();
        }
        //println!("Computed until detect");
    }

    pub fn load(&mut self, rt: &tokio::runtime::Runtime, rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>, tx_detections: tokio::sync::mpsc::UnboundedSender<Option<(ImageDetections, SendableMat)>>) -> Option<&JoinHandle<()>> {
        if let Some(handle) = &self.handle {
            if !handle.is_finished() {
                return None;
            }
        }

        self.handle = Some(rt.spawn(AsyncDetector::load_async(tx_detections, rx_capture, self.model.clone())));

        self.handle.as_ref()
    }

    pub fn _is_finished(&self) -> bool {
        if self.handle.is_none() {
            return true;
        }

        self.handle.as_ref().unwrap().is_finished()
    }

    pub fn kill(&mut self) {
        self.handle.as_ref().unwrap().abort()
    }

}

