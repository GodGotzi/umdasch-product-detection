#pragma once

#include <opencv2/highgui.hpp>
#include "logging/logging.hpp"
#include "perspective.hpp"

#define CAMERACAPTURE_TRY 5

class Watcher {
private:
	Perspective perspective;
	CameraTable* cameraTable;
	application_log::Logger* logger;

	cv::Mat watchSingle(cv::Size size);
	cv::Mat watchStereo(cv::Size size);
public:
	Watcher(CameraTable* cameraTable, Perspective perspective, application_log::Logger*);
	cv::Mat capture(cv::Size size);
};