#include <iostream>
#include <string>
#include <dlfcn.h>
#include <filesystem>

using namespace std;

namespace fs = std::filesystem;

int main()
{
    const char* libProfilers = "libprofilers.so";
    const char* libProfilersCopy = "libprofilerscopy.so";

    printf("Start\n");

    fs::remove(libProfilersCopy);
    fs::copy_file(libProfilers, libProfilersCopy);

    void *h = dlopen(libProfilersCopy, RTLD_LAZY | RTLD_GLOBAL);
    void *d = dlsym(h, "DllGetClassObject");
    //dlclose(h);

    printf("Overwrite\n");
    fs::remove(libProfilersCopy);
    fs::copy_file(libProfilers, libProfilersCopy);

    void *h2 = dlopen(libProfilersCopy, RTLD_LAZY | RTLD_GLOBAL);
    void *d2 = dlsym(h2, "DllGetClassObject");

    printf("End\n");
}