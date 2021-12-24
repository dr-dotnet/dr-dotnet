#pragma once

// AutoLock takes the function of std::lock_guard<>, as it can't be easily included because of the <mutex> header
// failing compilation on Linux when used with the PAL

template<typename TLock>
struct AutoLock {
	AutoLock(TLock& lock) : _lock(lock) {
		_lock.lock();
	}
	~AutoLock() {
		_lock.unlock();
	}

private:
	TLock& _lock;
};

