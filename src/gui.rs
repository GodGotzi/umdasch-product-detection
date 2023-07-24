pub mod detection;
pub mod terminal;
pub mod training;
pub mod utils;

use std::time::Instant;

use egui::{RichText, Vec2, ColorImage};
use egui_extras::{Size, RetainedImage};
use egui_grid::GridBuilder;
use image::{Rgba, ImageBuffer};

use crate::application::ProductDetectionApplication;

pub fn show_top_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top-panel").exact_height(20.0).show(ctx, |ui| {

        GridBuilder::new().new_row_align(Size::remainder(), egui::Align::Center)
            .cell(Size::remainder())
            .cell(Size::remainder())
            .show(ui, |mut grid| {
                grid.cell(|ui| {
                    let text = RichText::new("Umdasch - ProductDetectionApplication").strong();
                    ui.label(text);
                });

                grid.cell(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let text = RichText::new("@Elias Gottsbacher").strong();
                        ui.label(text);  
                    });
                });
            });
    });
}

pub fn show_main_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    show_training_panel(app, ctx, frame);
    show_terminal_panel(app, ctx, frame);

    /*
    egui::TopBottomPanel::top("detection-info-panel")
        .exact_height(20.0)
        .show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.heading("Detection");
                });
        });
    */

    utils::with_heading_panel("Detection", ctx);
    show_detection_panel(app, ctx, frame);

    egui::CentralPanel::default().show(ctx, |ui| {
        app.detection_context.img_width.set(ui.available_width());
        app.detection_context.img_height.set(ui.available_height());

        if let Some(img) = &app.detection_context.img {
            app.detection_context.img_init.set(true);

            if app.detection_context.img_init.changed() || app.detection_context.img_width.changed() || app.detection_context.img_height.changed() {
                if app.detection_context.resize_handle.is_none() && !app.detection_context.resizing {
                    app.detection_context.resizing = true;

                    let (tx, rx) = tokio::sync::oneshot::channel();

                    app.detection_context.resize_handle = Some(
                        app.tokio_runtime.spawn(resize_detection_img_async(img.clone(), (ui.available_width() as u32, ui.available_height() as u32), tx))
                    );

                    app.detection_context.resize_rx = Some(rx);
                }
            }

        } else {
            app.detection_context.img_init.set(false);
        }

        if let Some(img) = app.detection_context.resized_img.as_ref() {

            ui.image(img.texture_id(ctx), Vec2::new(img.width() as f32, img.height() as f32));
        }

    });

}

async fn resize_detection_img_async(img: ImageBuffer<Rgba<u8>, Vec<u8>>, size: (u32, u32), tx: tokio::sync::oneshot::Sender<RetainedImage>) {
    let img = 
        image::imageops::resize(
            &img,
            size.0,
            size.1,
            image::imageops::FilterType::Nearest);

    let color_img = ColorImage::from_rgba_premultiplied([img.dimensions().0 as usize, img.dimensions().1 as usize], &img);

    match tx.send(RetainedImage::from_color_image("detection-img", color_img)) {
        Ok(_) => println!("Resized"),
        Err(_) => panic!("Error while resizing"),
    }

}

//fn calculate_expansion(ctx)

fn show_training_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x * 0.225;

    egui::SidePanel::left("training-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            utils::with_heading("Training", ui);

            training::show(app, ui);
        });
}

fn show_terminal_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.y * 0.3;
    
    egui::TopBottomPanel::bottom("terminal-panel")
        .exact_height(exp)
        .resizable(false)
        .show(ctx, |ui| {
            terminal::show(app, ui);

        });
}

fn show_detection_panel(app: &mut ProductDetectionApplication, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let exp = frame.info().window_info.size.x  * 0.225;

    egui::SidePanel::right("detection-panel")
        .exact_width(exp)
        .resizable(false)
        .show(ctx, |ui| {
            detection::show(app, ui);
        });
}

