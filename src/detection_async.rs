use opencv::{prelude::*, videoio::{VideoCapture, VideoCaptureProperties}, imgcodecs::imwrite, core::Vector, imgproc::INTER_LINEAR};

use async_recursion::async_recursion;
use egui::mutex::Mutex;
use tokio::sync::watch::*;

use crate::detection_render::{model::YoloModel, self};

pub struct Detection {
    reload_sender: Option<Sender<bool>>,
    enabled: bool
}

impl Detection {

    pub async fn run(detection: &Mutex<Detection>) {
        Detection::load(detection).await;
    }

    #[async_recursion]
    pub async fn load(detection: &Mutex<Detection>) {
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

                println!("{}", raw_frame.rows());
                println!("{}", raw_frame.cols());
                let cropped_mat = raw_frame.apply(
                    opencv::core::Range::new(0, raw_frame.rows()).unwrap(), 
                    opencv::core::Range::new((raw_frame.cols() - raw_frame.rows()) / 2, raw_frame.cols() - ((raw_frame.cols() - raw_frame.rows()) / 2)).unwrap()).unwrap();

                let mut frame = Mat::default();


                opencv::imgproc::resize(&cropped_mat, &mut frame, opencv::core::Size::new(2048, 2048), 0.0, 0.0, INTER_LINEAR).unwrap();
                println!("{}", frame.rows());
                println!("{}", frame.cols());

                let detections = model.detect(frame.clone(), 0.2, 0.45).unwrap();

                match detection_render::render_detections(&mut frame, opencv::core::Size::new(2048, 2048), &detections) {
                    Ok(_) => println!("Detections drawn"),
                    Err(err) => println!("Detecions couldn't be drawn! {}", err)
                }

                let params: Vector<i32> = Vector::new();
                imwrite("frame.png", &frame, &params).unwrap();
            }
        });

        detection.lock().reload_sender = Some(tx);

        handle.await.unwrap();

        if detection.lock().enabled {
            Detection::load(detection).await;
        }
    }

}

impl Detection {

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

