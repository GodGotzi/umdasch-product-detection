
/*
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::highgui::*;
*/

use application::ProductDetectionApplication;
use egui::mutex::Mutex;

use tokio::runtime;
use detection::async_detector::AsyncDetector;
use lazy_static::lazy_static;

mod gui;
mod application;
mod detection;

lazy_static! {
    static ref DETECTION: Mutex<AsyncDetector> = Mutex::new(AsyncDetector::new());
}

fn main() -> Result<(), eframe::Error> {

    let (num_tokio_worker_threads, max_tokio_blocking_threads) = (num_cpus::get(), 512); // 512 is tokio's current default
    //println!("{}", num_tokio_worker_threads);
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .worker_threads(num_tokio_worker_threads)
        .max_blocking_threads(max_tokio_blocking_threads)
        .build().unwrap();

    let handle = AsyncDetector::run(rt, &DETECTION);

    if let Err(err) = handle {
        panic!("Couldn't create Receiver for DetectionContext {}", err);
    }

    let (rx_detections, rx_image, _detection_handle) = 
        handle.unwrap();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 1024.0)),
        fullscreen: true,
        icon_data: Some(load_icon("resource/icon.png")),
        ..Default::default()
    };

    let application = ProductDetectionApplication::new(rx_detections, rx_image);

    eframe::run_native(
        "Umdasch - Product Detection Application",
        options,
        Box::new(|_cc| Box::new(application)),
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







