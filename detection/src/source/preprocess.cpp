#include "preprocess.hpp"

namespace pre_proc {
	cv::Mat resize(cv::Mat& source, cv::Size &size) {
		cv::Mat resized;
		resize(source, resized, size, cv::INTER_LINEAR);

		return resized;
	}

	cv::Mat cropWidth(cv::Mat& source) {
		return source(
			cv::Range(0, source.rows),
			cv::Range(
				(source.cols - source.rows) / 2,
				source.cols - ((source.cols - source.rows) / 2)
			)
		);
	}

	cv::Mat preprocess(cv::Mat& source, cv::Size resize_to, cv::dnn::dnn4_v20220524::Net* net) {
		cv::Mat cropped = cropWidth(source);
		cv::Mat resized = resize(cropped, resize_to);
		cv::Mat blob = convertToBlob(resized);

		net->setInput(blob);

		return resized;
	}

	cv::Mat convertToBlob(cv::Mat& source) {
		cv::Mat blob;
		cv::dnn::blobFromImage(source, blob, 1. / 255, cv::Size(BLOB_WIDTH, BLOB_HEIGHT), cv::Scalar(), true, false);

		return blob;
	}

	std::vector<std::string> relevantClassNames(std::string path) {
		std::vector<std::string> names;
		
		std::ifstream ifs(path);
		std::string line;
		while (getline(ifs, line))
		{
			names.push_back(line);
		}

		return names;
	}

}