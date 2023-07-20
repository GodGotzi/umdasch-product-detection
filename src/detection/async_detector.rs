use opencv::{
    prelude::*, 
    videoio::{
        VideoCapture, 
        VideoCaptureProperties
    }, 
    imgproc::INTER_LINEAR, Error
};

use async_recursion::async_recursion;
use egui::mutex::Mutex;

use tokio::{
    sync::watch::*, 
    runtime::Runtime, task::JoinHandle
};

use crate::detection::{model::YoloModel, data::ImageDetections};

pub struct AsyncDetector {
    reload_sender: Option<Sender<bool>>,
    enabled: bool
}

pub struct SendableMat(Mat);

unsafe impl Send for SendableMat {}
unsafe impl Sync for SendableMat {}

impl AsyncDetector {

    pub fn run(rt: Runtime, detection: &'static Mutex<AsyncDetector>) -> 
        Result<(Receiver<Option<ImageDetections>>, Receiver<Option<SendableMat>>,  JoinHandle<()>), Error> {
        
        let (tx_detections, rx_detections) = channel(None);
        let (tx_image, rx_image) = channel(None);

        let handle = rt.spawn(AsyncDetector::load(tx_detections, tx_image, detection));

        Ok((rx_detections, rx_image, handle))
    }

    #[async_recursion]
    pub async fn load(tx_detections: Sender<Option<ImageDetections>>, tx_image: Sender<Option<SendableMat>>, detection: &'static Mutex<AsyncDetector>) {
        let (tx, rx) = channel(true);
    
        let handle = tokio::spawn(async move {

            let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32, 3840.0).unwrap();
            cam.set(VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32, 2080.0).unwrap();

            let opened = cam.is_opened().unwrap();

            if !opened {
                panic!("Couldn't find Webcamera!");
            }

            let mut model = YoloModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap();
            let mut raw_frame = Mat::default();

            while *rx.borrow() {
                let _result = cam.read(&mut raw_frame).unwrap();

                let cropped_mat = raw_frame.apply(
                    opencv::core::Range::new(0, raw_frame.rows()).unwrap(), 
                    opencv::core::Range::new((raw_frame.cols() - raw_frame.rows()) / 2, raw_frame.cols() - ((raw_frame.cols() - raw_frame.rows()) / 2)).unwrap()).unwrap();

                let mut frame = Mat::default();


                opencv::imgproc::resize(&cropped_mat, &mut frame, opencv::core::Size::new(2048, 2048), 0.0, 0.0, INTER_LINEAR).unwrap();

                let detections = model.detect(frame.clone(), 0.2, 0.45).unwrap();

                crate::detection::render::render_detections(&mut frame, opencv::core::Size::new(2048, 2048), &detections).unwrap();

                tx_detections.send(Some(detections)).unwrap();
                tx_image.send(Some(SendableMat(frame))).unwrap();
            }

            (tx_detections, tx_image)
        });

        detection.lock().reload_sender = Some(tx);

        let (tx_detections, tx_image, ) = handle.await.unwrap();

        if detection.lock().enabled {
            AsyncDetector::load(tx_detections, tx_image, detection).await;
        }
    }

}

impl AsyncDetector {

    pub const fn new() -> Self {
        Self {
            reload_sender: None,
            enabled: true
        }
    }

    pub fn _reload(&mut self) {
        match self.reload_sender.as_mut().unwrap().send(false) {
            Ok(_) => {
                println!("Reloaded");
            },
            Err(_) => todo!("Handle Error"),
        };

    }

}

