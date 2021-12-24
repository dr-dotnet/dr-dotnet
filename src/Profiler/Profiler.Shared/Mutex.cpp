#include "Mutex.h"

#ifdef _WINDOWS
#include <Windows.h>

struct Mutex::Impl {
	Impl() {
		::InitializeCriticalSection(&_cs);
	}

	void lock() {
		::EnterCriticalSection(&_cs);
	}

	void unlock() {
		::LeaveCriticalSection(&_cs);
	}

private:
	CRITICAL_SECTION _cs;
};

#else
#include <mutex>

struct Mutex::Impl {
	void lock() {
		_mutex.lock();
	}

	void unlock() {
		_mutex.unlock();
	}

private:
	std::mutex _mutex;
};
#endif

Mutex::Mutex() : _impl(new Impl) {}
Mutex::~Mutex() = default;

void Mutex::lock() {
	_impl->lock();
}

void Mutex::unlock() {
	_impl->unlock();
}

