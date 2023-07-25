use application::{ProductDetectionApplication, CaptureContainer};

use monitor::{async_detector::{AsyncDetector, SendableMat}, data::ImageDetections};
use tokio::runtime;

mod gui;
mod application;
mod monitor;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let (num_tokio_worker_threads, max_tokio_blocking_threads) = (num_cpus::get(), 512); // 512 is tokio's current default
    //println!("{}", num_tokio_worker_threads);
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .worker_threads(num_tokio_worker_threads)
        .max_blocking_threads(max_tokio_blocking_threads)
        .build().unwrap();
    
    let (tx_capture, rx_capture) = tokio::sync::watch::channel::<Option<((), SendableMat)>>(None);
    let (tx_detections, rx_detections) = tokio::sync::mpsc::unbounded_channel::<Option<(ImageDetections, SendableMat)>>();

    let container = CaptureContainer::new(tx_capture, rx_capture, tx_detections, rx_detections);

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 1024.0)),
        icon_data: Some(load_icon("resource/icon.png")),
        ..Default::default()
    };

    eframe::run_native(
        "Umdasch - Product Detection Application",
        options,
        Box::new(move |cc| {
            Box::new(ProductDetectionApplication::new(rt, AsyncDetector::new(), cc, container))
        }
    )).unwrap();

    Ok(())
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







