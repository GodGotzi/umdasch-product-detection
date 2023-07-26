

use std::task::Poll;

use egui::ColorImage;
use egui_extras::RetainedImage;
use image::ImageBuffer;
use opencv::{core::{Size, Point3_}, prelude::{Mat, MatTraitConst}, imgproc::INTER_LINEAR};
use tokio::task::JoinHandle;

use crate::product::ProductServer;

use self::{data::ImageDetections, async_detector::SendableMat};

pub mod async_detector;
pub mod render;
pub mod model;
pub mod data;

pub struct MonitorHandler {
    pub capture: ImageSubscriberCapture<()>,
    pub detection: ImageSubscriberDetection<ImageDetections>,
}

impl MonitorHandler {

    pub fn new(rx_capture: tokio::sync::watch::Receiver<Option<((), SendableMat)>>, rx_detection: tokio::sync::mpsc::UnboundedReceiver<Option<(ImageDetections, SendableMat)>>) -> Self {

        let capture_sub = ImageSubscriberCapture::<()>
            ::new(rx_capture, resize_capture);

        let detection_sub = ImageSubscriberDetection::<ImageDetections>
            ::new(rx_detection, resize_detection);

        Self { capture: capture_sub, detection: detection_sub }
    }

    pub fn kill(&mut self) {
        if let Some(handle) = self.capture.image_resize_handle.as_ref() {
            handle.abort();
        }

        if let Some(handle) = self.detection.image_resize_handle.as_ref() {
            handle.abort();
        }
    }

}

pub fn resize_capture(_: &(), matrix: &SendableMat, size: Size) -> RetainedImage {
    let min = std::cmp::min(size.width, size.height);

    let mut new_matrix = Mat::default();

    opencv::imgproc::resize(&matrix.0, &mut new_matrix, opencv::core::Size::new(min, min), 0.0, 0.0, INTER_LINEAR).unwrap();
    
    //println!("Image buffer creation");
    let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(min as u32, min as u32, |x, y| {
        let color = new_matrix.at_2d::<Point3_<u8>>(x as i32, y as i32).unwrap();
        image::Rgba([color.z, color.y, color.x, 255])
    });

    let mut img = 
        image::imageops::resize(
            &img,
            min as u32,
            min as u32,
            image::imageops::FilterType::Nearest);

    img = image::imageops::flip_vertical(&img);
    img = image::imageops::rotate90(&img);

    let color_img = ColorImage::from_rgba_premultiplied([img.dimensions().0 as usize, img.dimensions().1 as usize], &img);
    
    RetainedImage::from_color_image("detection-img", color_img)
}

pub fn resize_detection(detections: &ImageDetections, matrix: &SendableMat, size: Size, product_server: ProductServer) -> RetainedImage {
    let min = std::cmp::min(size.width, size.height);

    let mut new_matrix = Mat::default();

    opencv::imgproc::resize(&matrix.0, &mut new_matrix, opencv::core::Size::new(min, min), 0.0, 0.0, INTER_LINEAR).unwrap();
    
    render::render_detections(&mut new_matrix, opencv::core::Size::new(min, min), &product_server, &detections).unwrap();

    //println!("Image buffer creation");
    let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(min as u32, min as u32, |x, y| {
        let color = new_matrix.at_2d::<Point3_<u8>>(x as i32, y as i32).unwrap();
        image::Rgba([color.z, color.y, color.x, 255])
    });

    let mut img = 
        image::imageops::resize(
            &img,
            min as u32,
            min as u32,
            image::imageops::FilterType::Nearest);

    img = image::imageops::flip_vertical(&img);
    img = image::imageops::rotate90(&img);

    let color_img = ColorImage::from_rgba_premultiplied([img.dimensions().0 as usize, img.dimensions().1 as usize], &img);

    RetainedImage::from_color_image("detection-img", color_img)
}

pub struct ImageSubscriberDetection<T> {
    matrix_receiver: tokio::sync::mpsc::UnboundedReceiver<Option<(T, SendableMat)>>,
    matrix_bundle: Option<(T, SendableMat)>,
    image: Option<RetainedImage>,
    image_resize_handle: Option<JoinHandle<()>>,
    image_receiver: Option<tokio::sync::oneshot::Receiver<RetainedImage>>,
    resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size, product_server: ProductServer) -> RetainedImage,
    resizing: bool
}

impl <T: Sync + Send + Clone + 'static> ImageSubscriberDetection<T> {

    pub fn new(matrix_receiver: tokio::sync::mpsc::UnboundedReceiver<Option<(T, SendableMat)>>, resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size, product_server: ProductServer) -> RetainedImage) -> Self {
        Self { matrix_receiver: matrix_receiver, matrix_bundle: None, image: None, image_resize_handle: None, image_receiver: None, resize_fn: resize_fn, resizing: false }
    }

    pub fn check(&mut self) {
        let waker = futures::task::noop_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        if let Poll::Ready(Some(bundle)) = self.matrix_receiver.poll_recv(&mut ctx) {
            if let Some(bundle) = bundle.as_ref() {
                self.matrix_bundle = Some((bundle.0.clone(), bundle.1.clone()));
            }
        }

        if let Some(receiver) = self.image_receiver.as_mut() {
            if let Ok(image) = receiver.try_recv() {
                self.image = Some(image);

                self.image_receiver = None;
                self.image_resize_handle = None;
                self.resizing = false;
            }
        }

    }

    pub fn get_detections(&self) -> Option<&T> {
        if let Some(bundle) = self.matrix_bundle.as_ref() {
            return Some(&bundle.0);
        }

        None
    }

    pub fn is_bundle_arrived(&self) -> bool {
        self.matrix_bundle.is_some()
    }

    pub fn get_image(&self) -> Option<&RetainedImage> {
        self.image.as_ref()
    }

    pub fn resize(&mut self, size: Size, product_server: &ProductServer) {
        if !self.resizing {
            if let Some(bundle) = self.matrix_bundle.as_ref() {
                self.resizing = true;

                let (tx, rx) = tokio::sync::oneshot::channel();
    
                self.image_resize_handle = Some(tokio::spawn(
                    ImageSubscriberDetection::<T>::async_resize(bundle.0.clone(), bundle.1.clone(), size, product_server.clone(), self.resize_fn.clone(), tx))
                );
    
                self.image_receiver = Some(rx);
            }
        }
    }

    async fn async_resize(ctx: T, matrix: SendableMat, size: Size, product_server: ProductServer, resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size, product_server: ProductServer) -> RetainedImage, tx: tokio::sync::oneshot::Sender<RetainedImage>) {
        let image = resize_fn(&ctx, &matrix, size, product_server);

        if let Err(_) = tx.send(image) { }
    }
    
}

pub struct ImageSubscriberCapture<T> {
    matrix_receiver: tokio::sync::watch::Receiver<Option<(T, SendableMat)>>,
    matrix_bundle: Option<(T, SendableMat)>,
    image: Option<RetainedImage>,
    image_resize_handle: Option<JoinHandle<()>>,
    image_receiver: Option<tokio::sync::oneshot::Receiver<RetainedImage>>,
    resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size) -> RetainedImage,
    resizing: bool
}

impl <T: Sync + Send + Clone + 'static> ImageSubscriberCapture<T> {

    pub fn new(matrix_receiver: tokio::sync::watch::Receiver<Option<(T, SendableMat)>>, resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size) -> RetainedImage) -> Self {
        Self { matrix_receiver: matrix_receiver, matrix_bundle: None, image: None, image_resize_handle: None, image_receiver: None, resize_fn: resize_fn, resizing: false }
    }

    pub fn check(&mut self) {
        let bundle = self.matrix_receiver.borrow_and_update();

        if let Some(bundle) = bundle.as_ref() {
            self.matrix_bundle = Some((bundle.0.clone(), bundle.1.clone()));
        }

        if let Some(receiver) = self.image_receiver.as_mut() {
            if let Ok(image) = receiver.try_recv() {
                self.image = Some(image);
                self.image_receiver = None;
                self.image_resize_handle = None;
                self.resizing = false;
            }
        }
    }

    pub fn get_image(&self) -> Option<&RetainedImage> {
        self.image.as_ref()
    }

    pub fn resize(&mut self, size: Size) {
        if !self.resizing {
            if let Some(bundle) = self.matrix_bundle.as_ref() {
                self.resizing = true;

                let (tx, rx) = tokio::sync::oneshot::channel();
    
                self.image_resize_handle = Some(tokio::spawn(
                    ImageSubscriberCapture::<T>::async_resize(bundle.0.clone(), bundle.1.clone(), size, self.resize_fn.clone(), tx))
                );
    
                self.image_receiver = Some(rx);
            }
        }
    }

    async fn async_resize(ctx: T, matrix: SendableMat, size: Size, resize_fn: fn(ctx: &T, mat: &SendableMat, size: Size) -> RetainedImage, tx: tokio::sync::oneshot::Sender<RetainedImage>) {
        let image = resize_fn(&ctx, &matrix, size);

        if let Err(_) = tx.send(image) {
            println!("Resizing failed!");
        }
    }
    
}




