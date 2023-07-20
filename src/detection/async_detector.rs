
use opencv::{
    prelude::*, 
    videoio::{
        VideoCapture, 
        VideoCaptureProperties
    }, 
    imgproc::INTER_LINEAR
};

use tokio::sync::watch::*;

use crate::detection::{model::DNNModel, data::ImageDetections};

pub struct AsyncDetector;

pub struct SendableMat(Mat);

unsafe impl Send for SendableMat {}
unsafe impl Sync for SendableMat {}

impl AsyncDetector {

    pub async fn load(
        tx_detections: Sender<Option<ImageDetections>>, 
        tx_image: Sender<Option<SendableMat>>, 
        tx_reload: tokio::sync::mpsc::UnboundedSender<bool>,
        mut rx_reload: tokio::sync::mpsc::UnboundedReceiver<bool>, 
        mut rx_enable: Receiver<bool>) {

        let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

        cam.set(VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32, 3840.0).unwrap();
        cam.set(VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32, 2080.0).unwrap();

        let opened = cam.is_opened().unwrap();

        if !opened {
            panic!("Couldn't find Webcamera!");
        }

        while *rx_enable.borrow_and_update() {
            let mut model = DNNModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap();
            let mut raw_frame = Mat::default();

            tx_reload.send(true).unwrap();

            while rx_reload.blocking_recv().unwrap() {
                let _result = cam.read(&mut raw_frame).unwrap();
                println!("Hey hey hey");

                let cropped_mat = raw_frame.apply(
                    opencv::core::Range::new(0, raw_frame.rows()).unwrap(), 
                    opencv::core::Range::new((raw_frame.cols() - raw_frame.rows()) / 2, raw_frame.cols() - ((raw_frame.cols() - raw_frame.rows()) / 2)).unwrap()).unwrap();
    
                let mut frame = Mat::default();
    
                opencv::imgproc::resize(&cropped_mat, &mut frame, opencv::core::Size::new(2048, 2048), 0.0, 0.0, INTER_LINEAR).unwrap();
    
                let detections = model.detect(frame.clone(), 0.2, 0.45).unwrap();
    
                crate::detection::render::render_detections(&mut frame, opencv::core::Size::new(2048, 2048), &detections).unwrap();
    
                tx_detections.send(Some(detections)).unwrap();
                tx_image.send(Some(SendableMat(frame))).unwrap();
                println!("Hey hey hey!");

                
            }
        }

    }

}

