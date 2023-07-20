use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// Representation of an image with it's detections.
pub struct ImageDetections {
    // Array of [YoloDetection]s.
    pub detections: Vec<Detection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Representation of an object detection within an image.
pub struct Detection {
    // Top-Left Bounds Coordinate in X-Axis
    pub x: f32,

    // Top-Left Bounds Coordinate in Y-Axis
    pub y: f32,

    // Width of Bounding Box
    pub width: f32,

    // Height of Bounding Box
    pub height: f32,

    // Class Index
    pub class_index: u32,

    // Softmaxed Activation
    pub confidence: f32,
}

impl Detection {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}