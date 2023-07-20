use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDetections {
    pub detections: Vec<Detection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub class_index: u32,
    pub confidence: f32,
}

impl Detection {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}