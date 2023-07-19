
/*
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::highgui::*;
*/

use std::{thread, time::Duration};

use application::ProductDetectionApplication;
use egui::mutex::Mutex;
use yolo::Detection;
use lazy_static::lazy_static;

mod gui;
mod application;
mod yolo;

lazy_static! {
    static ref DETECTION: Mutex<yolo::Detection> = Mutex::new(yolo::Detection::new());
}

fn main() -> Result<(), eframe::Error> {
    /*
    let mut vid = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

    
    loop {
        
        vid.read(&mut frame).unwrap();

        imshow("frame", &frame).unwrap();
        wait_key(5).unwrap();        
    }
    */

    let handle = tokio::spawn(Detection::run(&DETECTION));
    thread::sleep(Duration::from_millis(5000));
    DETECTION.lock().reload();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 1024.0)),
        fullscreen: true,
        icon_data: Some(load_icon("resource/icon.png")),
        ..Default::default()
    };

    eframe::run_native(
        "Umdasch - Product Detection Application",
        options,
        Box::new(|_cc| Box::<ProductDetectionApplication>::default()),
    )
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}







