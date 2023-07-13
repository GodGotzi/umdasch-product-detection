
#include "logging/subscriber.hpp"

using namespace application_log;

Subscriber::Subscriber(std::ostream* output, std::string(*format) (LogLevel, std::string, time_t*)) {
	this->format = format;
	this->output = output;
}

void Subscriber::writeToSubscribtion(const LogLevel level, const std::string msg, time_t* time) {
	std::string formatted = (*this->format)(level, msg, time);
	(*this->output) << formatted << std::endl;
	this->output->flush();
}

void FileSubscriber::writeToSubscribtion(const LogLevel level, const std::string msg, time_t* time) {
	std::ofstream* output = (std::ofstream*)this->output;
	output->open(this->file_path, std::ios::out);

	std::string formatted = (*this->format)(level, msg, time);
	(*output) << formatted << std::endl;
	output->flush();
	output->close();
}

FileSubscriber::FileSubscriber(const std::string file_path, std::string(*format) (LogLevel, std::string, time_t*)) {
	this->format = format;
	this->file_path = file_path;

	std::ofstream* output = new std::ofstream(file_path, std::ios::out);
	this->output = output;
}