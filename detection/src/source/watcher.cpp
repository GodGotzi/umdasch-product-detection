#include "todo.h"

#include "watcher.hpp"
#include <opencv2/opencv.hpp>
#include <opencv2/core/core.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/videoio/videoio.hpp>
#include <iostream>

Watcher::Watcher(CameraTable* cameraTable, Perspective perspective, application_log::Logger*) {
	this->cameraTable = cameraTable;
	this->perspective = perspective;
	this->logger = logger;
}

cv::Mat Watcher::capture(cv::Size size) {
	
	switch (perspective) {
		case SINGLE:
			return watchSingle(size);
		case STEREO:
			return watchStereo(size);
		default:
			this->logger->warning("Invalid Perspective Stop Process" + perspective);
			exit(0);
	}

}

cv::Mat Watcher::watchSingle(cv::Size size) {
	cv::Mat output;

	int id = this->cameraTable->middleID;
	cv::VideoCapture capture;
	capture.open(id);

	capture.set(cv::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH, size.width);
	capture.set(cv::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT, size.height);

	if (!capture.isOpened()) {
		this->logger->warning("Couldn't open camera with ID:" + id);
		exit(0);
	}

	capture.read(output);
	
	return output;
}

cv::Mat Watcher::watchStereo(cv::Size size) {
	TODO("Stereo Perspective is not yet implemented")
}
