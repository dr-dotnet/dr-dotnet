#include "Common.h"
#include "Logger.h"
#include "CoreProfilerFactory.h"
#include "CoreProfiler.h"
#include <new>
#include <iostream>

HRESULT __stdcall CoreProfilerFactory::QueryInterface(REFIID riid, void** ppvObject) {

	Logger::Info(__FUNCTION__);

	if (ppvObject == nullptr)
		return E_POINTER;

	if (riid == __uuidof(IUnknown) || riid == __uuidof(IClassFactory)) {
		*ppvObject = static_cast<IClassFactory*>(this);
		return S_OK;
	}
	return E_NOINTERFACE;
}

ULONG __stdcall CoreProfilerFactory::AddRef(void) {
	return 2;
}

ULONG __stdcall CoreProfilerFactory::Release(void) {
	return 1;
}

HRESULT __stdcall CoreProfilerFactory::CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObject) {

	Logger::Info(__FUNCTION__);

	auto profiler = new (std::nothrow) CoreProfiler;
	if (profiler == nullptr)
		return E_OUTOFMEMORY;

	auto hr = profiler->QueryInterface(riid, ppvObject);
	profiler->Release();

	return hr;
}
