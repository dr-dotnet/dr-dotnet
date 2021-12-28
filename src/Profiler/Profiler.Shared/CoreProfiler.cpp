#include <assert.h>
#include <vector>
#include <string>
#include <iostream>
#include "CoreProfiler.h"
#include "Logger.h"
#include "OS.h"

#define TRY(x)  { \
					HRESULT hr; \
					if (FAILED(hr = x)) { \
						printf("Failed: '%s'. Reason: 0x%x.\n", #x, hr); \
						return hr; \
					} \
				}

HRESULT __stdcall CoreProfiler::QueryInterface(REFIID riid, void** ppvObject) {

	Logger::Info(__FUNCTION__);

	if (ppvObject == nullptr)
		return E_POINTER;

	//LPOLESTR pStr = NULL;
	//StringFromIID(riid, &pStr);
	//wprintf(L"?-> %s", pStr);

	//LPOLESTR pStr2 = NULL;
	//StringFromIID(__uuidof(ICorProfilerCallback2), &pStr2);
	//wprintf(L"2-> %s", pStr2);

	if (riid == __uuidof(IUnknown) ||
		riid == __uuidof(ICorProfilerCallback) ||
		riid == __uuidof(ICorProfilerCallback2) ||
		riid == __uuidof(ICorProfilerCallback3) ||
		riid == __uuidof(ICorProfilerCallback4) ||
		riid == __uuidof(ICorProfilerCallback5) ||
		riid == __uuidof(ICorProfilerCallback6) ||
		riid == __uuidof(ICorProfilerCallback7) ||
		riid == __uuidof(ICorProfilerCallback8) ||
		riid == __uuidof(ICorProfilerCallback9) ||
		riid == __uuidof(ICorProfilerCallback10) ||
		riid == __uuidof(ICorProfilerCallback11)) {
		AddRef();
		*ppvObject = static_cast<ICorProfilerCallback3*>(this);

		wprintf(L"OK");

		return S_OK;
	}

	wprintf(L"FAIL");


	return E_NOINTERFACE;
}

ULONG __stdcall CoreProfiler::AddRef(void) {
	return ++_refCount;
}

ULONG __stdcall CoreProfiler::Release(void) {
	auto count = --_refCount;
	if (count == 0)
		delete this;

	return count;
}

HRESULT CoreProfiler::Initialize(IUnknown* pICorProfilerInfoUnk) {

	Logger::Info(__FUNCTION__);

	pICorProfilerInfoUnk->QueryInterface(&_info);
	assert(_info);

	_info->SetEventMask(
		COR_PRF_MONITOR_MODULE_LOADS |
		COR_PRF_MONITOR_ASSEMBLY_LOADS |
		COR_PRF_MONITOR_GC |
		COR_PRF_MONITOR_CLASS_LOADS |
		COR_PRF_MONITOR_THREADS |
		COR_PRF_MONITOR_EXCEPTIONS |
		COR_PRF_MONITOR_JIT_COMPILATION |
		COR_PRF_MONITOR_OBJECT_ALLOCATED |
		COR_PRF_ENABLE_OBJECT_ALLOCATED);

	return S_OK;
}

HRESULT CoreProfiler::Shutdown() {
	Logger::Info("Profiler shutdown (PID=%d)", OS::GetPid());
	_info.Release();

	return S_OK;
}

HRESULT CoreProfiler::AppDomainCreationStarted(AppDomainID appDomainId) {
	return S_OK;
}

HRESULT CoreProfiler::AppDomainCreationFinished(AppDomainID appDomainId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::AppDomainShutdownStarted(AppDomainID appDomainId) {
	return S_OK;
}

HRESULT CoreProfiler::AppDomainShutdownFinished(AppDomainID appDomainId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::AssemblyLoadStarted(AssemblyID assemblyId) {
	return S_OK;
}

HRESULT CoreProfiler::AssemblyLoadFinished(AssemblyID assemblyId, HRESULT hrStatus) {
	WCHAR name[512];
	ULONG size;
	AppDomainID ad;
	ModuleID module;
	if (SUCCEEDED(_info->GetAssemblyInfo(assemblyId, sizeof(name) / sizeof(name[0]), &size, name, &ad, &module))) {
		Logger::Info("Assembly loaded: %s (id=0x%p)", OS::UnicodeToAnsi(name).c_str(), assemblyId);
	}

	return S_OK;
}

HRESULT CoreProfiler::AssemblyUnloadStarted(AssemblyID assemblyId) {
	return S_OK;
}

HRESULT CoreProfiler::AssemblyUnloadFinished(AssemblyID assemblyId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::ModuleLoadStarted(ModuleID moduleId) {
	return S_OK;
}

HRESULT CoreProfiler::ModuleLoadFinished(ModuleID moduleId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::ModuleUnloadStarted(ModuleID moduleId) {
	return S_OK;
}

HRESULT CoreProfiler::ModuleUnloadFinished(ModuleID moduleId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::ModuleAttachedToAssembly(ModuleID moduleId, AssemblyID AssemblyId) {
	return S_OK;
}

HRESULT CoreProfiler::ClassLoadStarted(ClassID classId) {
	return S_OK;
}

HRESULT CoreProfiler::ClassLoadFinished(ClassID classId, HRESULT hrStatus) {
	ModuleID module;
	mdTypeDef type;
	if (SUCCEEDED(_info->GetClassIDInfo(classId, &module, &type))) {
		auto name = GetTypeName(type, module);
		Logger::Info("Type %s loaded", name.c_str());
	}

	return S_OK;
}

HRESULT CoreProfiler::ClassUnloadStarted(ClassID classId) {
	return S_OK;
}

HRESULT CoreProfiler::ClassUnloadFinished(ClassID classId, HRESULT hrStatus) {
	return S_OK;
}

HRESULT CoreProfiler::FunctionUnloadStarted(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::JITCompilationStarted(FunctionID functionId, BOOL fIsSafeToBlock) {
	Logger::Info("JIT compilation started: %s", GetMethodName(functionId).c_str());

	return S_OK;
}

HRESULT CoreProfiler::JITCompilationFinished(FunctionID functionId, HRESULT hrStatus, BOOL fIsSafeToBlock) {
	Logger::Info("JIT compilation finished: %s", GetMethodName(functionId).c_str());

	return S_OK;
}

HRESULT CoreProfiler::JITCachedFunctionSearchStarted(FunctionID functionId, BOOL* pbUseCachedFunction) {
	return S_OK;
}

HRESULT CoreProfiler::JITCachedFunctionSearchFinished(FunctionID functionId, COR_PRF_JIT_CACHE result) {
	return S_OK;
}

HRESULT CoreProfiler::JITFunctionPitched(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::JITInlining(FunctionID callerId, FunctionID calleeId, BOOL* pfShouldInline) {
	return S_OK;
}

HRESULT CoreProfiler::ThreadCreated(ThreadID threadId) {
	Logger::Info("Thread 0x%p created", threadId);

	return S_OK;
}

HRESULT CoreProfiler::ThreadDestroyed(ThreadID threadId) {
	Logger::Info("Thread 0x%p destroyed", threadId);

	return S_OK;
}

HRESULT CoreProfiler::ThreadAssignedToOSThread(ThreadID managedThreadId, DWORD osThreadId) {
	Logger::Info("Thread 0x%p assigned to OS thread %d", managedThreadId, osThreadId);
	return S_OK;
}

HRESULT CoreProfiler::RemotingClientInvocationStarted() {
	return S_OK;
}

HRESULT CoreProfiler::RemotingClientSendingMessage(GUID* pCookie, BOOL fIsAsync) {
	return S_OK;
}

HRESULT CoreProfiler::RemotingClientReceivingReply(GUID* pCookie, BOOL fIsAsync) {
	return S_OK;
}

HRESULT CoreProfiler::RemotingClientInvocationFinished() {
	return S_OK;
}

HRESULT CoreProfiler::RemotingServerReceivingMessage(GUID* pCookie, BOOL fIsAsync) {
	return S_OK;
}

HRESULT CoreProfiler::RemotingServerInvocationStarted() {
	return S_OK;
}

HRESULT CoreProfiler::RemotingServerInvocationReturned() {
	return S_OK;
}

HRESULT CoreProfiler::RemotingServerSendingReply(GUID* pCookie, BOOL fIsAsync) {
	return S_OK;
}

HRESULT CoreProfiler::UnmanagedToManagedTransition(FunctionID functionId, COR_PRF_TRANSITION_REASON reason) {
	Logger::Verbose(__FUNCTION__);
	return S_OK;
}

HRESULT CoreProfiler::ManagedToUnmanagedTransition(FunctionID functionId, COR_PRF_TRANSITION_REASON reason) {
	Logger::Verbose(__FUNCTION__);
	return S_OK;
}

HRESULT CoreProfiler::RuntimeSuspendStarted(COR_PRF_SUSPEND_REASON suspendReason) {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeSuspendFinished() {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeSuspendAborted() {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeResumeStarted() {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeResumeFinished() {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeThreadSuspended(ThreadID threadId) {
	return S_OK;
}

HRESULT CoreProfiler::RuntimeThreadResumed(ThreadID threadId) {
	return S_OK;
}

HRESULT CoreProfiler::MovedReferences(ULONG cMovedObjectIDRanges, ObjectID* oldObjectIDRangeStart, ObjectID* newObjectIDRangeStart, ULONG* cObjectIDRangeLength) {
	return S_OK;
}

HRESULT CoreProfiler::ObjectAllocated(ObjectID objectId, ClassID classId) {
	ModuleID module;
	mdTypeDef type;
	if (SUCCEEDED(_info->GetClassIDInfo(classId, &module, &type))) {
		auto name = GetTypeName(type, module);
		if(!name.empty())
			Logger::Info("Allocated object 0x%p of type %s", objectId, name.c_str());
	}
	return S_OK;
}

HRESULT CoreProfiler::ObjectsAllocatedByClass(ULONG cClassCount, ClassID* classIds, ULONG* cObjects) {
	return S_OK;
}

HRESULT CoreProfiler::ObjectReferences(ObjectID objectId, ClassID classId, ULONG cObjectRefs, ObjectID* objectRefIds) {
	return S_OK;
}

HRESULT CoreProfiler::RootReferences(ULONG cRootRefs, ObjectID* rootRefIds) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionThrown(ObjectID thrownObjectId) {
	Logger::Info("Exception");
	ClassID classid;
	HR(_info->GetClassFromObject(thrownObjectId, &classid));
	ModuleID module;
	mdTypeDef type;
	HR(_info->GetClassIDInfo(classid, &module, &type));
	Logger::Info("Exception %s thrown", GetTypeName(type, module).c_str());

	std::vector<std::string> data;
	if (SUCCEEDED(_info->DoStackSnapshot(0, StackSnapshotCB, 0, &data, nullptr, 0))) {
		// TODO
	}

	return S_OK;
}

HRESULT CoreProfiler::ExceptionSearchFunctionEnter(FunctionID functionId) {
	Logger::Info(__FUNCTION__);
	return S_OK;
}

HRESULT CoreProfiler::ExceptionSearchFunctionLeave() {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionSearchFilterEnter(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionSearchFilterLeave() {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionSearchCatcherFound(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionOSHandlerEnter(UINT_PTR __unused) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionOSHandlerLeave(UINT_PTR __unused) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionUnwindFunctionEnter(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionUnwindFunctionLeave() {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionUnwindFinallyEnter(FunctionID functionId) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionUnwindFinallyLeave() {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionCatcherEnter(FunctionID functionId, ObjectID objectId) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionCatcherLeave() {
	Logger::Info(__FUNCTION__);
	return S_OK;
}

HRESULT CoreProfiler::COMClassicVTableCreated(ClassID wrappedClassId, const GUID& implementedIID, void* pVTable, ULONG cSlots) {
	return S_OK;
}

HRESULT CoreProfiler::COMClassicVTableDestroyed(ClassID wrappedClassId, const GUID& implementedIID, void* pVTable) {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionCLRCatcherFound() {
	return S_OK;
}

HRESULT CoreProfiler::ExceptionCLRCatcherExecute() {
	Logger::Info(__FUNCTION__);
	return S_OK;
}

HRESULT CoreProfiler::ThreadNameChanged(ThreadID threadId, ULONG cchName, WCHAR* name) {
	return S_OK;
}

HRESULT CoreProfiler::GarbageCollectionStarted(int cGenerations, BOOL* generationCollected, COR_PRF_GC_REASON reason) {
	Logger::Info(__FUNCTION__);
	Logger::Info("GC started. Gen0=%s, Gen1=%s, Gen2=%s",
		generationCollected[0] ? "Yes" : "No", generationCollected[1] ? "Yes" : "No", generationCollected[2] ? "Yes" : "No");

	return S_OK;
}

HRESULT CoreProfiler::SurvivingReferences(ULONG cSurvivingObjectIDRanges, ObjectID* objectIDRangeStart, ULONG* cObjectIDRangeLength) {
	return S_OK;
}

HRESULT CoreProfiler::GarbageCollectionFinished() {
	Logger::Info("GC finished");

	return S_OK;
}

HRESULT CoreProfiler::FinalizeableObjectQueued(DWORD finalizerFlags, ObjectID objectID) {
	return S_OK;
}

HRESULT CoreProfiler::RootReferences2(ULONG cRootRefs, ObjectID* rootRefIds, COR_PRF_GC_ROOT_KIND* rootKinds, COR_PRF_GC_ROOT_FLAGS* rootFlags, UINT_PTR* rootIds) {
	return S_OK;
}

HRESULT CoreProfiler::HandleCreated(GCHandleID handleId, ObjectID initialObjectId) {
	return S_OK;
}

HRESULT CoreProfiler::HandleDestroyed(GCHandleID handleId) {
	return S_OK;
}

HRESULT CoreProfiler::InitializeForAttach(IUnknown* pICorProfilerInfoUnk, void* pvClientData, UINT cbClientData) {
	Logger::Info(__FUNCTION__);

	pICorProfilerInfoUnk->QueryInterface(&_info);
	assert(_info);

	TRY(_info->SetEventMask(
		COR_PRF_MONITOR_MODULE_LOADS |
		COR_PRF_MONITOR_ASSEMBLY_LOADS |
		COR_PRF_MONITOR_GC |
		COR_PRF_MONITOR_CLASS_LOADS |
		COR_PRF_MONITOR_THREADS |
		COR_PRF_MONITOR_EXCEPTIONS |
		COR_PRF_MONITOR_JIT_COMPILATION));

	Logger::Info("ATTACHED!");

	return S_OK;
}

HRESULT CoreProfiler::ProfilerAttachComplete() {

	Logger::Info(__FUNCTION__);

	return S_OK;
}

HRESULT CoreProfiler::ProfilerDetachSucceeded() {
	return S_OK;
}

//HRESULT CoreProfiler::ReJITCompilationStarted(FunctionID functionId, ReJITID rejitId, BOOL fIsSafeToBlock) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::GetReJITParameters(ModuleID moduleId, mdMethodDef methodId, ICorProfilerFunctionControl* pFunctionControl) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::ReJITCompilationFinished(FunctionID functionId, ReJITID rejitId, HRESULT hrStatus, BOOL fIsSafeToBlock) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::ReJITError(ModuleID moduleId, mdMethodDef methodId, FunctionID functionId, HRESULT hrStatus) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::MovedReferences2(ULONG cMovedObjectIDRanges, ObjectID* oldObjectIDRangeStart, ObjectID* newObjectIDRangeStart, SIZE_T* cObjectIDRangeLength) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::SurvivingReferences2(ULONG cSurvivingObjectIDRanges, ObjectID* objectIDRangeStart, SIZE_T* cObjectIDRangeLength) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::ConditionalWeakTableElementReferences(ULONG cRootRefs, ObjectID* keyRefIds, ObjectID* valueRefIds, GCHandleID* rootIds) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::GetAssemblyReferences(const WCHAR* wszAssemblyPath, ICorProfilerAssemblyReferenceProvider* pAsmRefProvider) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::ModuleInMemorySymbolsUpdated(ModuleID moduleId) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::DynamicMethodJITCompilationStarted(FunctionID functionId, BOOL fIsSafeToBlock, LPCBYTE pILHeader, ULONG cbILHeader) {
//	return S_OK;
//}
//
//HRESULT CoreProfiler::DynamicMethodJITCompilationFinished(FunctionID functionId, HRESULT hrStatus, BOOL fIsSafeToBlock) {
//	return S_OK;
//}

std::string CoreProfiler::GetTypeName(mdTypeDef type, ModuleID module) const {
	CComPtr<IMetaDataImport> spMetadata;
	if (SUCCEEDED(_info->GetModuleMetaData(module, ofRead, IID_IMetaDataImport, reinterpret_cast<IUnknown**>(&spMetadata)))) {
		WCHAR name[256];
		ULONG nameSize = 256;
		DWORD flags;
		mdTypeDef baseType;
		if (SUCCEEDED(spMetadata->GetTypeDefProps(type, name, 256, &nameSize, &flags, &baseType))) {
			return OS::UnicodeToAnsi(name);
		}
	}
	return "";
}

std::string CoreProfiler::GetMethodName(FunctionID function) const {
	Logger::Info(__FUNCTION__);

	ModuleID module;
	mdToken token;
	mdTypeDef type;
	ClassID classId;
	if (FAILED(_info->GetFunctionInfo(function, &classId, &module, &token)))
		return "";

	CComPtr<IMetaDataImport> spMetadata;
	if (FAILED(_info->GetModuleMetaData(module, ofRead, IID_IMetaDataImport, reinterpret_cast<IUnknown**>(&spMetadata))))
		return "";
	PCCOR_SIGNATURE sig;
	ULONG blobSize, size, attributes;
	WCHAR name[256];
	DWORD flags;
	ULONG codeRva;
	if (FAILED(spMetadata->GetMethodProps(token, &type, name, 256, &size, &attributes, &sig, &blobSize, &codeRva, &flags)))
		return "";

	return GetTypeName(type, module) + "::" + OS::UnicodeToAnsi(name);
}

HRESULT __stdcall CoreProfiler::StackSnapshotCB(FunctionID funcId, UINT_PTR ip, COR_PRF_FRAME_INFO frameInfo,
	ULONG32 contextSize, BYTE context[], void* clientData) {
	// TODO
	return S_OK;
}

