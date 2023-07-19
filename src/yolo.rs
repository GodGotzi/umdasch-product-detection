pub mod data;
pub mod model;

use std::{time, thread};

use async_recursion::async_recursion;
use egui::mutex::Mutex;
use tokio::sync::watch::*;

use self::model::YoloModel;

pub struct Detection {
    yolomodel: Option<YoloModel>,
    reload_sender: Option<Sender<bool>>,
    enabled: bool
}

unsafe impl Sync for Detection {
    
}

impl Detection {

    pub const fn new() -> Self {

        Self {
            yolomodel: None,
            reload_sender: None,
            enabled: true
        }
    }

    pub async fn run(detection: &Mutex<Detection>) {
        Detection::load(detection).await;
    }

    #[async_recursion]
    pub async fn load(detection: &Mutex<Detection>) {
        let (tx, rx) = channel(true);

        let handle = tokio::spawn(async move {

            while *rx.borrow() {
                println!("Running...");
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

    pub fn reload(&mut self) {
        match self.reload_sender.as_mut().unwrap().send(false) {
            Ok(_) => {
                println!("Reloaded");
            },
            Err(_) => todo!("Handle Error"),
        };

    }

}

