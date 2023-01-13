#include "Logger.h"
#include "OS.h"
#include <sstream>
#include <assert.h>
#include <time.h>
#include <iomanip>

Logger& Logger::Get() {
	static Logger logger;
	return logger;
}

void Logger::Shutdown() {
	Get().Term();
}

const char* Logger::LogLevelToString(LogLevel level) {
	switch (level) {
		case LogLevel::Verbose: return "Verbose";
		case LogLevel::Debug: return "Debug";
		case LogLevel::Info: return "Info";
		case LogLevel::Warning: return "Warning";
		case LogLevel::Error: return "Error";
		case LogLevel::Fatal: return "Fatal";
	}
	assert(false);
	return "Unknown";
}

LogLevel Logger::GetLevel() const {
	return _level;
}

void Logger::SetLevel(LogLevel level) {
	_level = level;
}

void Logger::Term() {
	if (_file.is_open()) {
		_file.flush();
		_file.close();
	}
}

#ifdef _WINDOWS
#include <Windows.h>
#endif
#include <iostream>

void Logger::DoLog(LogLevel level, const char* text) {

	// build message with time, level, pid, tid, text
	char time[48];
	const auto now = ::time(nullptr);
#ifdef _WINDOWS
	tm lt;
	localtime_s(&lt, &now);
	auto plt = &lt;
#else
	auto plt = localtime(&now);
#endif
	timespec ts;
	timespec_get(&ts, TIME_UTC);
	
	strftime(time, sizeof(time), "%D %T", plt);

	std::cout << "[Profiler] " << text << std::endl;
}

Logger::Logger() {

}
