// product-detection.cpp: Definiert den Einstiegspunkt für die Anwendung.
//

#include <filesystem>

#include "product-detection.h"
#include "preprocess.hpp"
#include "detector.hpp"

using namespace cv;
using namespace std;
using namespace dnn;
using namespace application_log;

Logger* createPipelineLogger(std::string state);
std::string pipe_format(LogLevel lvl, std::string msg, time_t* time);
void printResult(cv::Mat &img, std::vector<Detection>* detections, std::string result_pic, Logger* logger);

int main(int argc, char** argv) {
	if (argc != 5) {
		cout << "ERROR:ARGS_INVALID" << endl;
		exit(0);
	}

	
	std::string yolo_net_path = argv[0];
	std::string names_path = argv[1];
	std::string result_pic_path = argv[2];
	std::string interface_log_path = argv[3];
	std::string log_path = argv[4];

	Logger* interface_logger = createPipelineLogger(interface_log_path);
	Logger* logger = LoggerFactory::generateLoggerWithFileSubscriberAndDefaultSubscriber(log_path);

    std::vector<std::string> class_names = pre_proc::relevantClassNames(names_path);

	cv::dnn::dnn4_v20220524::Net net = dnn::readNet(yolo_net_path);
	CameraTable table = { -1, -1, 1 };

	Watcher watcher(&table, Perspective::SINGLE, logger);
	Mat capture = watcher.capture(cv::Size(3840, 2080));

	Mat preproc = pre_proc::preprocess(capture, cv::Size(2048, 2048), &net);

    vector<Mat> detection_matrixes;
    net.forward(detection_matrixes, net.getUnconnectedOutLayersNames());

	std::vector<Detection> detections;

	Detector detector(&detection_matrixes, &detections);
	Mat result = detector.processDetections(preproc, class_names);

	//debug_detections(&detections, &class_names, interface_logger);
	printResult(result, &detections, result_pic_path, interface_logger);

	return 0;
}

Logger* createPipelineLogger(std::string path) {
	Subscriber sub(&std::cout, pipe_format);

	Logger* logger = LoggerFactory::generateLoggerWithFileSubscriber(path);
	logger->add_subscriber(sub);

	return logger;
}

std::string pipe_format(LogLevel lvl, std::string msg, time_t* time) {
	std::string lvl_str = formatLevel(lvl);

	return lvl_str + ":" + msg;
}

void printResult(cv::Mat &img, std::vector<Detection>* detections, std::string result_pic, Logger* logger) {
	
	for (size_t i = 0; i < detections->size(); i++) {
		Detection detection = (*detections)[i];
		std::string detection_str = std::format(
			"product_id={};confidence={};box=[x={},y={},width={},height={}]", 
			detection.product_id, 
			detection.confidence, 
			detection.box.x, detection.box.y, detection.box.width, detection.box.height);
		logger->log(LogLevel::RESULT, detection_str);
	}

	cv::imwrite(result_pic, img);
}
