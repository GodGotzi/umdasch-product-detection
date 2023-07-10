// product-detection.cpp: Definiert den Einstiegspunkt für die Anwendung.
//

#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <iostream>

#include "product-detection.h"

using namespace cv;
using namespace std;

Mat src_gray;
const int thresh = 30;
RNG rng(12345);

void thresh_callback(int, void*);

int main(int argc, char** argv)
{
	Mat src;

	VideoCapture capture(0);

	while (true) {

		capture >> src;

		if (src.empty())
		{
			cout << "Could not open or find the camera!\n" << endl;
			return -1;
		}

		cvtColor(src, src_gray, COLOR_BGR2GRAY);
		blur(src_gray, src_gray, Size(3, 3));

		const char* source_window = "Source";

		thresh_callback(0, 0);
		waitKey(5);
	}

	return 0;
}

void thresh_callback(int, void*)
{
	Mat canny_output;

	Canny(src_gray, canny_output, thresh, thresh * 2);

	vector<vector<Point> > contours;
	vector<Vec4i> hierarchy;
	findContours(canny_output, contours, hierarchy, RETR_TREE, CHAIN_APPROX_SIMPLE);
	Mat drawing = Mat::zeros(canny_output.size(), CV_8UC3);

	for (size_t i = 0; i < contours.size(); i++)
	{
		Scalar color = Scalar(rng.uniform(0, 256), rng.uniform(0, 256), rng.uniform(0, 256));
		drawContours(drawing, contours, (int)i, color, 2, LINE_8, hierarchy, 0);
	}

	imshow("Contours", drawing);
}