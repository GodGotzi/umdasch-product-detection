#pragma once

#include <opencv2/opencv.hpp>
#include <vector>

enum Perspective {
	SINGLE,
	STEREO
};

struct CameraTable {
	int leftID;
	int rightID;
	int middleID;
};

void stereo(cv::Mat &ouput, cv::Mat img1, cv::Mat img2);