
use std::{collections::HashMap, sync::{Mutex, Arc}};

use opencv::{
    core::{copy_make_border, Scalar, BORDER_CONSTANT, CV_32F},
    dnn::read_net_from_onnx,
    prelude::{Mat, MatTraitConst, NetTrait, NetTraitConst},
    Error,
};

use crate::training::TrainingManager;

use super::data::*;


fn _iou(a: &Detection, b: &Detection) -> f32 {
    let area_a = a._area();
    let area_b = b._area();

    let top_left = (a.x.max(b.x), a.y.max(b.y));
    let bottom_right = (a.x + a.width.min(b.width), a.y + a.height.min(b.height));

    let intersection =
        (bottom_right.0 - top_left.0).max(0.0) * (bottom_right.1 - top_left.1).max(0.0);

    intersection / (area_a + area_b - intersection)
}


fn suppression_fn(detections: Vec<Detection>) -> Vec<Detection> {
    let mut suppressed_detections: HashMap<u32, Detection> = HashMap::new();
    let mut sorted_detections: Vec<Detection> = detections.to_vec();

    sorted_detections.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
    sorted_detections.reverse();

    for sorted_detection in sorted_detections.iter() {
        if !suppressed_detections.contains_key(&sorted_detection.class_index) {
            suppressed_detections.insert(sorted_detection.class_index, sorted_detection.clone());
        }
    }

    let mut detections_vec: Vec<Detection> = suppressed_detections.values().into_iter().map(|detection| detection.clone()).collect();
    detections_vec.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
    detections_vec.reverse();
    detections_vec
}

/// Filter detections by confidence.
fn filter_confidence(detections: Vec<Detection>, min_confidence: f32) -> Vec<Detection> {
    detections
        .into_iter()
        .filter(|dsetection| dsetection.confidence >= min_confidence)
        .collect()
}

#[derive(Clone, Debug)]
pub struct DNNModel {
    net: opencv::dnn::Net,
    input_size: opencv::core::Size_<i32>,
}

impl DNNModel {
    
    pub fn new_from_file(model_path: &str, input_size: (i32, i32)) -> Result<Self, Error> {
        let net = read_net_from_onnx(model_path)?;
        DNNModel::new_from_network(net, input_size)
    }

    pub fn new_from_network(
        mut network: opencv::dnn::Net,
        input_size: (i32, i32),
    ) -> Result<Self, Error> {
        let cuda_count = opencv::core::get_cuda_enabled_device_count()?;

        if cuda_count > 0 {
            network.set_preferable_backend(opencv::dnn::DNN_BACKEND_CUDA)?;
            network.set_preferable_target(opencv::dnn::DNN_TARGET_CUDA)?;
        }

        Ok(Self {
            net: network,
            input_size: opencv::core::Size_::new(input_size.0, input_size.1),
        })
    }

    /// Detect objects in an image.
    fn forward(&mut self, blob: &Mat) -> Result<Mat, Error> {
        let mut output_tensor_blobs: opencv::core::Vector<Mat> = opencv::core::Vector::default();

        self.net.set_input(&blob, "", 1.0, Scalar::default())?;
        self.net.forward(
            &mut output_tensor_blobs,
            &self.net.get_unconnected_out_layers_names()?,
        )?;

        output_tensor_blobs.get(0)
    }

    /// Convert the output of the YOLOv5 model to a vector of [YoloDetection].
    fn convert_to_detections(&self, outputs: &Mat) -> Result<Vec<Detection>, Error> {
        let rows = *outputs.mat_size().get(1).unwrap();
        let mut detections = Vec::<Detection>::with_capacity(rows as usize);

        for row in 0..rows {
            let cx: &f32 = outputs.at_3d(0, row, 0)?;
            let cy: &f32 = outputs.at_3d(0, row, 1)?;
            let w: &f32 = outputs.at_3d(0, row, 2)?;
            let h: &f32 = outputs.at_3d(0, row, 3)?;
            let sc: &f32 = outputs.at_3d(0, row, 4)?;

            let mut x_min = *cx - *w / 2.0;
            let mut y_min = *cy - *h / 2.0;

            x_min /= self.input_size.width as f32;
            y_min /= self.input_size.height as f32;
            let mut width = *w / self.input_size.width as f32;
            let mut height = *h / self.input_size.height as f32;

            x_min = x_min.max(0.0).min(1_f32);
            y_min = y_min.max(0.0).min(1_f32);
            width = width.max(0.0).min(1_f32);
            height = height.max(0.0).min(1_f32);

            let mat_size = outputs.mat_size();
            let classes = *mat_size.get(2).unwrap() - 5;
            let mut classes_confidences = vec![];

            for j in 5..5 + classes {
                let confidence: &f32 = outputs.at_3d(0, row, j)?;
                classes_confidences.push(confidence);
            }

            let mut max_index = 0;
            let mut max_confidence = 0.0;
            for (index, confidence) in classes_confidences.iter().enumerate() {
                if *confidence > &max_confidence {
                    max_index = index;
                    max_confidence = **confidence;
                }
            }

            detections.push(Detection {
                x: x_min,
                y: y_min,
                width,
                height,
                class_index: max_index as u32,
                confidence: *sc,
                color: Scalar::new(0.0, 255.0, 255.0, 1.0)
            })
        }

        Ok(detections)
    }

    /// Run the model on an image and return the detections.
    pub fn detect(
        &mut self,
        frame: &Mat,
        minimum_confidence: f32,
        suppression: bool,
        filter_conf: bool,
        training_manager: Arc<Mutex<TrainingManager>>
    ) -> Result<ImageDetections, Error> {

        // Load the image
        training_manager.lock().unwrap().print("Loading Webcam capture");
        let mat = self.load_capture(frame)?;

        // Run the model on the image.
        training_manager.lock().unwrap().print("Running Model on capture");
        let result = self.forward(&mat)?;

        // Convert the result to a Vec of Detections.
        training_manager.lock().unwrap().print("Collect Detections");
        let mut detections = self.convert_to_detections(&result)?;

        // Filter the detections by confidence.
        if filter_conf {
            training_manager.lock().unwrap().print(format!("Filter Confidence of: {}", minimum_confidence).as_str());
            detections = filter_confidence(detections, minimum_confidence);
        }

        if suppression {
            training_manager.lock().unwrap().print("Suppressing Detections");
            detections = suppression_fn(detections);
        }

        // Non-maximum suppression.
        //let detections = suppression(detections);

        Ok(ImageDetections {
            detections,
        })
    }

    fn load_capture(&self, image: &Mat) -> Result<Mat, Error> { 

        let mut boxed_image = Mat::default();

        copy_make_border(
            image,
            &mut boxed_image,
            0,
            0,
            0,
            0,
            BORDER_CONSTANT,
            Scalar::new(114f64, 114f64, 114f64, 0f64),
        )?;

        let blob = opencv::dnn::blob_from_image(
            &boxed_image,
            1.0 / 255.0,
            opencv::core::Size_ {
                width: self.input_size.width,
                height: self.input_size.height,
            },
            Scalar::new(0f64, 0f64, 0f64, 0f64),
            true,
            false,
            CV_32F,
        )?;

        Ok(blob)
    }
    
}