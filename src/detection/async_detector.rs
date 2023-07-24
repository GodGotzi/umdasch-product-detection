use std::time::Duration;

use image::ImageBuffer;
use opencv::{
    prelude::*, 
    videoio::{
        VideoCapture, 
        VideoCaptureProperties
    }, 
    imgproc::INTER_LINEAR, core::Point3_,
};
use tokio::{task::{JoinHandle, JoinError}, sync::oneshot::Receiver};

use crate::detection::{model::DNNModel, data::ImageDetections};

pub struct AsyncDetector {
    handle: Option<JoinHandle<()>>,
    rx_result: Option<Receiver<(SendableImage, ImageDetections)>>,
    model: DNNModel
}

#[derive(Clone, Debug)]
pub struct SendableImage(pub ImageBuffer<image::Rgba<u8>, Vec<u8>>);

unsafe impl Send for SendableImage {}
unsafe impl Sync for SendableImage {}

impl AsyncDetector {

    pub fn new() -> Self {
        Self { 
            handle: None, 
            rx_result: None, 
            model: DNNModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap(),
        }
    }

    pub fn empty_or_value<T>(receiver: &mut tokio::sync::mpsc::UnboundedReceiver<T>, value: T) -> T {
        if let Ok(val) = receiver.try_recv() {
            return val;
        }

        value
    }


    async fn load_async(tx: tokio::sync::oneshot::Sender<(SendableImage, ImageDetections)>, mut model: DNNModel) {
        let now = std::time::Instant::now();
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

        //println!("Computed until detect");
        let detections = model.detect(frame.clone(), 0.6, 0.45).unwrap();
        
        //println!("Computed until render");
        crate::detection::render::render_detections(&mut frame, opencv::core::Size::new(2048, 2048), &detections).unwrap();

        //println!("Image buffer creation");
        let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2048, 2048, |x, y| {
            let color = frame.at_2d::<Point3_<u8>>(x as i32, y as i32).unwrap();
            image::Rgba([color.z, color.y, color.x, 255])
        });

        tx.send((SendableImage(img), detections)).unwrap();
        println!("elapsed {}", now.elapsed().as_millis());
    }

    pub fn load(&mut self, rt: &tokio::runtime::Runtime) -> Option<&JoinHandle<()>> {
        if let Some(handle) = &self.handle {
            if handle.is_finished() {
                return None;
            }
        }

        let (tx, rx) = tokio::sync::oneshot::channel();

        self.handle = Some(rt.spawn(AsyncDetector::load_async(tx, self.model.clone())));
        self.rx_result = Some(rx);

        self.handle.as_ref()
    }

    pub fn is_finished(&self) -> bool {
        if self.handle.is_none() {
            return true;
        }

        self.handle.as_ref().unwrap().is_finished()
    }

    pub fn result(&mut self) -> Option<(SendableImage, ImageDetections)> {
        if let Some(rx) = self.rx_result.as_mut() {
            return match rx.try_recv() {
                Ok(result) => Some(result),
                Err(_) => None,
            }    
        } else {
            None
        }
    }

    pub fn kill(&mut self) {
        self.handle.as_ref().unwrap().abort()
    }

}

