#pragma once

#include "preprocess.hpp"
#include "logging/logging.hpp"

#include <opencv2/opencv.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <vector>
#include <iostream>

const float SCORE_THRESHOLD = 0.5f;
const float NMS_THRESHOLD = 0.45f;
const float CONFIDENCE_THRESHOLD = 0.45f;

const float FONT_SCALE = 0.7f;
const int FONT_FACE = 0;
const int THICKNESS = 1;

struct Detection {
	int product_id;
	float confidence;
	cv::Rect box;
};

void draw_label(cv::Mat& input_image, std::string label, int left, int top);
void debug_detections(
	std::vector<Detection>* detections,
	std::vector<std::string>* class_names,
	application_log::Logger* logger);

class Detector {
private:
	std::vector<Detection>* detections;
	std::vector<cv::Mat>* detection_matrixes;
public:
	Detector(std::vector<cv::Mat>* detection_matrixes, std::vector<Detection>* detections);

	cv::Mat processDetections(cv::Mat& img, std::vector<std::string> class_names);

	std::vector<int> filteredDetectionIndices(
		std::vector<int>* class_ids,
		std::vector<float>* confidences, 
		std::vector<cv::Rect>* boxes);

	cv::Mat collectDetectionsAndDraw(
		cv::Mat& img, 
		std::vector<int>* indices, 
		std::vector<int>* class_ids, 
		std::vector<float>* confidences, 
		std::vector<cv::Rect>* boxes,
		std::vector<std::string>* class_names);

	std::vector<Detection>* getDetections();
};