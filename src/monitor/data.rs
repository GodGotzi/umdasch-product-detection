use opencv::core::Scalar;

#[derive(Debug, Clone)]
pub struct ImageDetections {
    pub detections: Vec<Detection>,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub class_index: u32,
    pub confidence: f32,
    pub color: Scalar
}

impl Detection {
    pub fn _area(&self) -> f32 {
        self.width * self.height
    }
}