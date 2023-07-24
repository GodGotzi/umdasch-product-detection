

use egui_extras::RetainedImage;
use image::ImageBuffer;
use tokio::task::JoinHandle;

use self::data::ImageDetections;

pub mod async_detector;
pub mod render;
pub mod model;
pub mod data;

#[derive(Default)]
pub struct DetectionContext {
    pub img: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub resized_img: Option<RetainedImage>,
    pub detections: Option<ImageDetections>,
    pub resize_handle: Option<JoinHandle<()>>,
    pub resize_rx: Option<tokio::sync::oneshot::Receiver<RetainedImage>>,
    pub resizing: bool,
    pub img_width: ChangeItem<f32>,
    pub img_height: ChangeItem<f32>,
    pub img_init: ChangeItem<bool>
}

impl DetectionContext {

    pub fn kill(&mut self) {
        if let Some(handle) = self.resize_handle.as_ref() {
            handle.abort();
        }
    }

}

#[derive(Default, Debug)]
pub struct ChangeItem<T> {
    last: T,
    current: T
}

impl <T: PartialEq + Default + Copy> ChangeItem<T> {

    pub fn changed(&self) -> bool {
        self.last != self.current
    }

    pub fn set(&mut self, new: T) {
        self.last = self.current;
        self.current = new;
    }

}