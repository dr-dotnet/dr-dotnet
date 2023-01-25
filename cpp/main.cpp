#include <iostream>
#include <string>
#include <dlfcn.h>
#include <filesystem>

using namespace std;

namespace fs = std::filesystem;

int main()
{
    printf("Start\n");

    fs::remove("libprofilerscopy.so");
    fs::copy_file("libprofilers.so", "libprofilerscopy.so");

    void *h = dlopen("./libprofilerscopy.so", RTLD_LAZY);
    void *d = dlsym(h, "DllGetClassObject");
    //dlclose(h);

    printf("Overwrite\n");
    fs::remove("libprofilerscopy.so");
    fs::copy_file("libprofilers.so", "libprofilerscopy.so");

    void *h2 = dlopen("./libprofilerscopy.so", RTLD_LAZY);
    void *d2 = dlsym(h2, "DllGetClassObject");

    printf("End\n");
}