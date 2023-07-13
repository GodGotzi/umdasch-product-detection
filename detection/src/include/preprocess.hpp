#pragma once

#include <opencv2/opencv.hpp>
#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <vector>
#include <fstream>

#define BLOB_WIDTH 640.0
#define BLOB_HEIGHT 640.0

namespace pre_proc {
	cv::Mat resize(cv::Mat& source, cv::Size& size);
	cv::Mat cropWidth(cv::Mat& source);
	cv::Mat preprocess(cv::Mat &source, cv::Size resize_to, cv::dnn::dnn4_v20220524::Net* net);
	cv::Mat convertToBlob(cv::Mat &source);
	std::vector<std::string> relevantClassNames(std::string path);
}
