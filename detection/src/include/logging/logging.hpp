#pragma once

#include <vector>
#include <ctime>
#include <iostream>

#include "level.h"
#include "subscriber.hpp"

namespace application_log {

	std::string formatLevel(LogLevel lvl);

	class Logger {
	private:
		std::vector<Subscriber> subscribers;
	public:
		Logger() {}
		void add_subscriber(Subscriber sub);
		void err(std::string msg);
		void info(std::string msg);
		void debug(std::string msg);
		void warning(std::string msg);
		void log(LogLevel lvl, std::string msg);
		void log(LogLevel lvl, std::string msg, Subscriber sub);
	};

	class LoggerFactory {
	private:
		static std::vector<Logger*> logger;
	public:
		static Logger* TODO_LOGGER;

		static Logger* generateDefaultLogger();

		static Logger* generateLoggerWithSubscriber(Subscriber sub);

		static Logger* generateLoggerWithFileSubscriber(std::string file_path);

		static Logger* generateLoggerWithFileSubscriberAndDefaultSubscriber(std::string file_path);
	};
};