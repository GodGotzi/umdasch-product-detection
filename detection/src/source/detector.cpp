#include "detector.hpp"

using namespace cv;
using namespace std;
using namespace dnn;

cv::Scalar BLACK = cv::Scalar(0, 0, 0);
cv::Scalar BLUE = cv::Scalar(255, 178, 50);
cv::Scalar YELLOW = cv::Scalar(0, 255, 255);
cv::Scalar RED = cv::Scalar(0, 0, 255);

Detector::Detector(std::vector<cv::Mat>* detection_matrixes, std::vector<Detection>* detections) {
	this->detection_matrixes = detection_matrixes;
    this->detections = detections;
}

cv::Mat Detector::processDetections(cv::Mat& img, std::vector<std::string> class_names) {

    vector<int> class_ids;
    vector<float> confidences;
    vector<Rect> boxes;
    // Resizing factor.

    float x_factor = (float) img.cols / (float) BLOB_WIDTH;
    float y_factor = (float) img.rows / (float) BLOB_HEIGHT;

    float* data = (float*) (*this->detection_matrixes)[0].data;
    const int dimensions = 85;
    // 25200 for default size 640.
    const int rows = 25200;
    // Iterate through 25200 detections.

    for (int i = 0; i < rows; ++i)
    {
        float confidence = data[4];
        // Discard bad detections and continue.
        if (confidence >= CONFIDENCE_THRESHOLD)
        {
            float* classes_scores = data + 5;
            // Create a 1x85 Mat and store class scores of 80 classes.
            cv::Mat scores(1, (int) class_names.size(), CV_32FC1, classes_scores);

            // Perform minMaxLoc and acquire the index of best class  score.
            Point class_id;
            double max_class_score;
            minMaxLoc(scores, 0, &max_class_score, 0, &class_id);
            // Continue if the class score is above the threshold.
            if (max_class_score > SCORE_THRESHOLD)
            {
                // Store class ID and confidence in the pre-defined respective vectors.
                confidences.push_back(confidence);
                class_ids.push_back(class_id.x);
                // Center.
                float cx = data[0];
                float cy = data[1];
                // Box dimension.
                float w = data[2];
                float h = data[3];
                // Bounding box coordinates.
                int left = int((cx - 0.5 * w) * x_factor);
                int top = int((cy - 0.5 * h) * y_factor);
                int width = int(w * x_factor);
                int height = int(h * y_factor);
                // Store good detections in the boxes vector.
                boxes.push_back(Rect(left, top, width, height));
            }
        }
        // Jump to the next row.
        data += 85;
    }

    std::vector<int> indicies = filteredDetectionIndices(&class_ids, &confidences, &boxes);
    
    return collectDetectionsAndDraw(img, &indicies, &class_ids, &confidences, &boxes, &class_names);
}

cv::Mat Detector::collectDetectionsAndDraw(
	cv::Mat &img,
    std::vector<int>* indices,
	std::vector<int>* class_ids,
	std::vector<float>* confidences,
	std::vector<cv::Rect>* boxes,
    std::vector<std::string>* class_names) {
	
    for (int i = 0; i < indices->size(); i++)
    {
        int idx = (*indices)[i];
        Rect box = (*boxes)[idx];
        int left = box.x;
        int top = box.y;
        int width = box.width;
        int height = box.height;
        // Draw bounding box.
        rectangle(img, Point(left, top), Point(left + width, top + height), BLUE, 3 * THICKNESS);
        // Get the label for the class name and its confidence.
        string label = std::format("%.2f", (*confidences)[idx]);
        label = (*class_names)[(*class_ids)[idx]] + ":" + label;
        // Draw class labels.
        draw_label(img, label, left, top);
    }

    return img;
	
}

std::vector<int> Detector::filteredDetectionIndices(
	std::vector<int>* class_ids, 
	std::vector<float>* confidences, 
	std::vector<cv::Rect>* boxes) {

	vector<int> indices;

    for (int i = 0; i < confidences->size(); i++) indices.push_back(i);

	//cv::dnn::NMSBoxes(*boxes, *confidences, SCORE_THRESHOLD, NMS_THRESHOLD, indices);

    return indices;
}

std::vector<Detection>* Detector::getDetections() {
    return this->detections;
}

void draw_label(Mat& input_image, std::string label, int left, int top)
{
    // Display the label at the top of the bounding box.
    int baseLine;
    Size label_size = getTextSize(label, FONT_FACE, FONT_SCALE, THICKNESS, &baseLine);
    top = max(top, label_size.height);
    // Top left corner.
    Point tlc = Point(left, top);
    // Bottom right corner.
    Point brc = Point(left + label_size.width, top + label_size.height + baseLine);
    // Draw white rectangle.
    rectangle(input_image, tlc, brc, BLACK, FILLED);
    // Put the label on the black rectangle.
    putText(input_image, label, Point(left, top + label_size.height), FONT_FACE, FONT_SCALE, YELLOW, THICKNESS);
}

void debug_detections(
    std::vector<Detection>* detections, 
    std::vector<std::string>* class_names, 
    application_log::Logger* logger) {

    for (size_t i = 0; i < detections->size(); i++) {
        Detection detection = (*detections)[i];
        Rect box = detection.box;

        logger->info(std::format("Detection({})[product_id={}, confidence={}, box=[x={},y={},width={},height={}]]", 
           i+1, detection.product_id, detection.confidence, box.x, box.y, box.width, box.height));
        
    }
}