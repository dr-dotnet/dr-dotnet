#pragma once

#include <string>
#include <common.h>

class OS final {
public:
	static std::string ReadEnvironmentVariable(const char* name);
	static int GetPid();
	static int GetTid();
	static std::string GetCurrentDir();
	static std::string UnicodeToAnsi(const WCHAR* str);
};

