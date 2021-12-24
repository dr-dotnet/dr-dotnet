#pragma once

#include <fstream>
#include "Mutex.h"
#include "AutoLock.h"


enum class LogLevel {
	Verbose,
	Debug,
	Info,
	Warning,
	Error,
	Fatal
};

class Logger final {
public:
	static Logger& Get();
	static void Shutdown();
	static const char* LogLevelToString(LogLevel level);

	LogLevel GetLevel() const;
	void SetLevel(LogLevel level);

	template<typename... Args>
	void Log(LogLevel level, Args&&... args) {
		if (level < _level)
			return;

		char buffer[1 << 10];
#ifdef _WINDOWS
		sprintf_s(buffer, args...);
#else
		sprintf(buffer, args...);
#endif
		DoLog(level, buffer);

	}
	template<typename... Args>
	__forceinline static void Info(Args&&... args) {
		Get().Log(LogLevel::Info, std::forward<Args>(args)...);
	}

	template<typename... Args>
	__forceinline static void Debug(Args&&... args) {
		Get().Log(LogLevel::Debug, std::forward<Args>(args)...);
	}

	template<typename... Args>
	__forceinline static void Error(Args&&... args) {
		Get().Log(LogLevel::Error, std::forward<Args>(args)...);
	}

	template<typename... Args>
	__forceinline static void Warning(Args&&... args) {
		Get().Log(LogLevel::Warning, std::forward<Args>(args)...);
	}

	template<typename... Args>
	__forceinline static void Verbose(Args&&... args) {
		Get().Log(LogLevel::Verbose, std::forward<Args>(args)...);
	}

private:
	Logger();
	void DoLog(LogLevel level, const char* text);
	void Term();

private:
	Mutex _lock;
	std::ofstream _file;
	LogLevel _level = LogLevel::Debug;
};

