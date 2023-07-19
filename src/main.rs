
/*
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use opencv::highgui::*;
*/

use application::ProductDetectionApplication;

mod gui;
mod application;

fn main() -> Result<(), eframe::Error> {
    /*
    let mut vid = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

    
    loop {
        let mut frame: Mat = Mat::default();
        
        vid.read(&mut frame).unwrap();

        imshow("frame", &frame).unwrap();
        wait_key(5).unwrap();        
    }
    */

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







