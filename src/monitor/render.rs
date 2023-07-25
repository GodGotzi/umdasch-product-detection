
use opencv::{
    imgproc::{get_text_size, put_text, FONT_HERSHEY_SIMPLEX, LineTypes, rectangle}, 
    prelude::Mat, 
    core::Scalar, 
    Error
};

use opencv::core::{Rect, Point};

use super::data::ImageDetections;

const FONT_SCALE: f64 = 0.5;
const FONT_FACE: i32 = FONT_HERSHEY_SIMPLEX;
const THICKNESS: i32 = 1;

const BLACK: Scalar  = Scalar::new(0.0, 0.0, 0.0, 1.0);
const _BLUE: Scalar = Scalar::new(255.0, 178.0, 50.0, 1.0);
const YELLOW: Scalar = Scalar::new(0.0, 255.0, 255.0, 1.0);
const WHITE: Scalar = Scalar::new(255.0, 255.0, 255.0, 1.0);

fn draw_label(img: &mut Mat, label: &str, left: i32, native_top: i32) -> Result<(), Error> {
    let mut baseline = 0;

    let size = get_text_size(label, FONT_FACE, FONT_SCALE, THICKNESS, &mut baseline)?;
    let top = std::cmp::max(native_top, size.height);

    let rect = Rect::new(left, top, size.width, size.height);

    if let Err(_) = rectangle(img, rect, 
        WHITE, -1, LineTypes::LINE_4 as i32, 0) {
        todo!("Handle Error")
    }

    let org = Point::new(left, top + size.height);

    put_text(img,
            label, org, FONT_FACE, 
            FONT_SCALE, BLACK, THICKNESS, 
            LineTypes::LINE_AA as i32, false)?;

    Ok(())
}

pub fn render_detections(img: &mut Mat, size: opencv::core::Size, detections: &ImageDetections) -> Result<(), Error> {

    for detection in detections.detections.iter() {
        
        let x = (detection.x) * size.width as f32;
        let y = (detection.y) * size.height as f32;
        let width = (detection.width) * size.width as f32;
        let height = (detection.height) * size.height as f32;

        let rect = Rect::new(x as i32, y as i32, width as i32, height as i32);

        println!("{:?}", rect);

        rectangle(img, rect, YELLOW, 3*THICKNESS, LineTypes::LINE_4 as i32, 0)?;

        let label: String = format!("{} | Conf: {:.3}", detection.class_index, detection.confidence);
        draw_label(img, label.as_str(), x as i32, y as i32)?;
    }

    Ok(())
}