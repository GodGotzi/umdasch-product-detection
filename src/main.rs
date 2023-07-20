
/*
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::highgui::*;
*/

use application::ProductDetectionApplication;
use egui::mutex::Mutex;

use tokio::runtime;
use detection_async::Detection;
use lazy_static::lazy_static;

mod gui;
mod application;
mod detection_async;
mod detection_render;

lazy_static! {
    static ref DETECTION: Mutex<detection_async::Detection> = Mutex::new(detection_async::Detection::new());
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

    let _handle = rt.spawn(Detection::run(&DETECTION));

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







