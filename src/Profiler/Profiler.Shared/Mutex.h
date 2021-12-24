#pragma once

#include <memory>

class Mutex {
public:
	Mutex();
	~Mutex();

	void lock();
	void unlock();

private:
	struct Impl;
	std::unique_ptr<Impl> _impl;
};


