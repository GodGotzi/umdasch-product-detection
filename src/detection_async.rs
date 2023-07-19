pub mod data;
pub mod model;

use std::{time, thread, path::Path};

use opencv::{prelude::*, videoio::VideoCapture};

use async_recursion::async_recursion;
use egui::mutex::Mutex;
use tokio::sync::watch::*;

use crate::detection_async::model::YoloModel;

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

            let opened = cam.is_opened().unwrap();

            if !opened {
                panic!("Couldn't find Webcamera!");
            }


            while *rx.borrow() {

                let mut frame = Mat::default();
                let _result = cam.read(&mut frame).unwrap();

                let mut model = YoloModel::new_from_file("yolov5/yolov5s.onnx", (2048, 2048)).unwrap();
                
                let detections = model.detect(frame, 0.05, 0.45).unwrap();
                
                println!("{:?}", detections);
            }

            println!("Waiting 2000ms");
            let millis = time::Duration::from_millis(2000);
            thread::sleep(millis);
           
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

    pub fn reload(&mut self) {
        match self.reload_sender.as_mut().unwrap().send(false) {
            Ok(_) => {
                println!("Reloaded");
            },
            Err(_) => todo!("Handle Error"),
        };

    }

}

