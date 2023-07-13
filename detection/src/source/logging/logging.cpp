#include <algorithm>
#include <iostream>
#include "logging/logging.hpp"

using namespace application_log;

std::string default_format(LogLevel lvl, std::string msg, tm* time);

std::string application_log::formatLevel(LogLevel lvl) {

	switch (lvl)
	{
		case INFO:
			return "INFO";
		case DEBUG:
			return "DEBUG";
		case WARNING:
			return "WARNING";
		case ERR:
			return "ERR";
		case RESULT:
			return "RESULT";
		case STATE:
			return "STATE";
		default:
			return "";
	}
}

void Logger::add_subscriber(Subscriber sub) {
	this->subscribers.push_back(sub);
}

void Logger::err(std::string msg) {
	this->log(LogLevel::ERR, msg);
}

void Logger::info(std::string msg) {
	this->log(LogLevel::INFO, msg);
}

void Logger::debug(std::string msg) {
	this->log(LogLevel::DEBUG, msg);

}

void Logger::warning(std::string msg) {
	this->log(LogLevel::WARNING, msg);
}

void Logger::log(LogLevel lvl, std::string msg) {
	time_t now = time(0);

	for (Subscriber sub : this->subscribers) {
		sub.writeToSubscribtion(lvl, msg, &now);
	}
}

void Logger::log(LogLevel lvl, std::string msg, Subscriber sub) {
	time_t now = time(0);

	sub.writeToSubscribtion(lvl, msg, &now);
}

std::string default_format(LogLevel lvl, std::string msg, time_t* time) {
	std::string lvl_str = formatLevel(lvl);
	char* time_cstr = ctime(time);
	std::string time_str = time_cstr;
	time_str = time_str.substr(0, time_str.length() - 1);

	return lvl_str + " " + time_str + " MSG: " + msg;
}

Logger* LoggerFactory::generateDefaultLogger() {
	Subscriber subscriber(&std::cout, default_format);
	Logger* logger = new Logger();
	logger->add_subscriber(subscriber);

	LoggerFactory::logger.push_back(logger);

	return logger;
}

Logger* LoggerFactory::generateLoggerWithSubscriber(Subscriber sub) {
	Logger* logger = new Logger();
	logger->add_subscriber(sub);

	LoggerFactory::logger.push_back(logger);

	return logger;
}

Logger* LoggerFactory::generateLoggerWithFileSubscriber(std::string file_path) {
	FileSubscriber subscriber(file_path, default_format);
	Logger* logger = new Logger();
	logger->add_subscriber(subscriber);

	LoggerFactory::logger.push_back(logger);

	return logger;

}

Logger* LoggerFactory::generateLoggerWithFileSubscriberAndDefaultSubscriber(std::string file_path) {
	Subscriber def_subscriber(&std::cout, default_format);
	FileSubscriber subscriber(file_path, default_format);
	Logger* logger = new Logger();
	logger->add_subscriber(subscriber);
	logger->add_subscriber(def_subscriber);

	LoggerFactory::logger.push_back(logger);

	return logger;

}

std::vector<Logger*> LoggerFactory::logger = std::vector<Logger*>();
Logger* LoggerFactory::TODO_LOGGER = LoggerFactory::generateLoggerWithFileSubscriberAndDefaultSubscriber("todo.log");