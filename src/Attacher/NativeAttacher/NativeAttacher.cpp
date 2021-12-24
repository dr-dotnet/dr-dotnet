#include <iostream>
#include "MetaHost.h"
#include "objidlbase.h"
#include "psapi.h"
#include <Windows.h> // Todo: make it cross platform
#include <tchar.h>

#pragma comment(lib, "mscoree.lib")

HANDLE GetProcessByID(const DWORD id)
{
	return OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, id);
}

HANDLE GetProcessByName(const TCHAR element[])
{
    DWORD aProcesses[1024], cbNeeded, cProcesses;
    if (!EnumProcesses(aProcesses, sizeof(aProcesses), &cbNeeded))
        return NULL;

    // Calculate how many process identifiers were returned.
    cProcesses = cbNeeded / sizeof(DWORD);

    // Print the name and process identifier for each process.
    for (unsigned int i = 0; i < cProcesses; i++)
    {
        DWORD dwProcessID = aProcesses[i];
        // Get a handle to the process.
        HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwProcessID);

        // Get the process name.
        TCHAR szEachProcessName[MAX_PATH];
        if (hProcess != NULL)
        {
            HMODULE hMod;
            DWORD cbNeeded;

            if (EnumProcessModules(hProcess, &hMod, sizeof(hMod), &cbNeeded))
            {
                GetModuleBaseName(hProcess, hMod, szEachProcessName, sizeof(szEachProcessName) / sizeof(TCHAR));
            }
        }

        // if they dont match, exit. otherwise get this party started
        if (_tcscmp(element, szEachProcessName) == 0) {
            return hProcess;
        }

        CloseHandle(hProcess);
    }

    return NULL;
}

void AttachToProcess(LPCWSTR sz_runtimeVersion, HANDLE handle) {
	ICLRMetaHost* pMetaHost = NULL;
	ICLRRuntimeInfo* pRuntimeInfo = NULL;
	ICLRProfiling* pClrProfiling = NULL;
	HRESULT hr;
	BOOL bLoadable;

	/* Get ICLRMetaHost instance */
	if (FAILED(CLRCreateInstance(CLSID_CLRMetaHost, IID_ICLRMetaHost, (VOID**)&pMetaHost))) {
		printf("[!] CLRCreateInstance(...) failed\n");
		return;
	}

	/* Get ICLRRuntimeInfo instance */
	if (FAILED(pMetaHost->GetRuntime(sz_runtimeVersion, IID_ICLRRuntimeInfo, (VOID**)&pRuntimeInfo))) {
		printf("[!] pMetaHost->GetRuntime(...) failed\n");
		return;
	}

	/* Check if the specified runtime can be loaded */
	if (FAILED(pRuntimeInfo->IsLoadable(&bLoadable)) || !bLoadable) {
		printf("[!] pRuntimeInfo->IsLoadable(...) failed\n");
		return;
	}

	if (FAILED(pRuntimeInfo->IsLoaded(handle, &bLoadable))) {
		printf("[!] This runtime is not loaded in this process\n");
		return;
	}

	/* Get ICLRProfiling instance */
	if (FAILED(hr = pRuntimeInfo->GetInterface(CLSID_CLRProfiling, IID_ICLRProfiling, (LPVOID*)&pClrProfiling))) {
		printf("[!] Failed getting profiling interface. Error: 0x % x.\n", hr);
		return;
	}

	DWORD id = GetProcessId(handle);

	BSTR GuidName = SysAllocString(L"{BD097ED8-733E-43FE-8ED7-A95FF9A8448C}");
	IID Guid;
	if (FAILED(IIDFromString(GuidName, &Guid)))
	{
		printf("[!] Failed retriving profiler GUID\n");
		return;
	}

	LPCWSTR path = L"C:\\Users\\oginiaux\\Projects\\DotNextMoscow2019\\x64\\Debug\\DotNext.Profiler.Windows.dll";

	if (FAILED(hr = pClrProfiling->AttachProfiler(id, 10000, &Guid, path, NULL, 0))) {
		printf("[!] Failed attaching profiler. Error: 0x%x.\n", hr); // file not found...
		return;
	}

	printf("ATTACHED!");
}

int main()
{
    std::cout << "Hello World!\n";

	HMODULE hModule = LoadLibrary(L"mscoree.dll");
	CreateInterfaceFnPtr createInterface = (CreateInterfaceFnPtr)GetProcAddress(hModule, "CreateInterface");

	ICLRMetaHost* pMetaHost = NULL;
	HRESULT hr = createInterface(CLSID_CLRMetaHost, IID_ICLRMetaHost, (LPVOID*)&pMetaHost);

    HANDLE processHandle = GetProcessByName(L"MyDotnetApplication.exe");

	//Installed: v2.0.50727
    //Installed: v4.0.30319
	AttachToProcess(L"v4.0.30319", processHandle);

	std::cin.get();

    if (processHandle == NULL) {
        std::cout << "Failed getting process handle!\n";
    }

	IEnumUnknown* pEnumUnknown = NULL;
	hr = pMetaHost->EnumerateLoadedRuntimes(processHandle, &pEnumUnknown);

    if (SUCCEEDED(hr)) {
        std::cout << "Failed getting loaded runtimes!\n";
    }

    pMetaHost->EnumerateInstalledRuntimes(&pEnumUnknown);

	IUnknown* enumRuntime = NULL;
	ICLRRuntimeInfo* runtimeInfo = NULL;
	LPWSTR frameworkName = NULL;
	DWORD bytes = 2048, result = 0;

    wprintf(L"Runtimes:\n");

	// Enumerate through runtimes and show supported frameworks
	while (pEnumUnknown->Next(1, &enumRuntime, 0) == S_OK) {
		if (enumRuntime->QueryInterface<ICLRRuntimeInfo>(&runtimeInfo) == S_OK) {
			if (runtimeInfo != NULL) {
				runtimeInfo->GetVersionString((LPWSTR)&frameworkName, &bytes);
				std::cout << "[*] Supported Framework: " << frameworkName;
                wprintf(L"Formatted message: %s\n", frameworkName);
			}
		}
	}

    ICorRuntimeHost* pRuntimeHost = NULL;

	std::cin.get();
}
