use application::{ProductDetectionApplication, CaptureContainer};

use monitor::{async_detector::{AsyncDetector, SendableMat}, data::ImageDetections};
use product::ProductServer;

mod gui;
mod application;
mod monitor;
mod product;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    //println!("{}", num_tokio_worker_threads);
    
    let (tx_capture, rx_capture) = tokio::sync::watch::channel::<Option<((), SendableMat)>>(None);
    let (tx_detections, rx_detections) = tokio::sync::mpsc::unbounded_channel::<Option<(ImageDetections, SendableMat)>>();

    let container = CaptureContainer::new(tx_capture, rx_capture, tx_detections, rx_detections);

    let mut product_server = ProductServer::new("products/".into());
    product_server.load().unwrap();

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
            Box::new(ProductDetectionApplication::new(AsyncDetector::new(), cc, container, product_server))
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







