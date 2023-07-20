use application::ProductDetectionApplication;

use tokio::{runtime, sync::watch::channel};
use detection::async_detector::AsyncDetector;

mod gui;
mod application;
mod detection;

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

    let (tx_detections, rx_detections) = channel(None);
    let (tx_image, rx_image) = channel(None);
    let (tx_reload, rx_reload) = tokio::sync::mpsc::unbounded_channel();
    let (tx_enable, rx_enable) = channel(true);

    let handle = rt.spawn(AsyncDetector::load(tx_detections, tx_image, tx_reload.clone(), rx_reload, rx_enable));

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 1024.0)),
        fullscreen: true,
        icon_data: Some(load_icon("resource/icon.png")),
        ..Default::default()
    };

    let application = ProductDetectionApplication::new(rx_detections, rx_image, tx_reload, tx_enable);

    eframe::run_native(
        "Umdasch - Product Detection Application",
        options,
        Box::new(|_cc| Box::new(application)),
    ).unwrap();

    handle.await.unwrap();

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







