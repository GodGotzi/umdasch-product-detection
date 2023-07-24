use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDetections {
    pub detections: Mutex<Vec<Detection>>,
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