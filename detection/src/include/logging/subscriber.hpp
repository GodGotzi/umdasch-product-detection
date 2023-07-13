#pragma once

#include <iostream>
#include <fstream>
#include <ctime>

#include "level.h"

namespace application_log {

	class Subscriber {
	protected:
		std::string(*format) (LogLevel, std::string, time_t*);
		std::ostream* output;
	public:
		Subscriber() {}
		Subscriber(std::ostream* output, std::string(*format) (LogLevel, std::string, time_t*));
		void writeToSubscribtion(LogLevel level, std::string msg, time_t* time);
	};

	class FileSubscriber : public Subscriber {
	private:
		std::string file_path;
	public:
		FileSubscriber(std::string file_path, std::string(*format) (LogLevel, std::string, time_t*));
		void writeToSubscribtion(LogLevel level, std::string msg, time_t* time);
	};

};