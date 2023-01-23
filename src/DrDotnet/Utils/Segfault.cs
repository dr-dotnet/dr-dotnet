using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;

namespace DrDotnet.Utils;

public static class Segfault
{
    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, out nint classFactoryPtr);
    private delegate void CreateInstance(nint self, nint pUnkOuter, ref Guid riid, out nint ppvObjectPtr);

    private static Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
    private static Guid iclassFactoryGuid = new Guid("00000001-0000-0000-c000-000000000046");
    private static Guid iCorProfilerCallback8Guid = new Guid("5BED9B15-C079-4D47-BFE2-215A140C07E0");

    public static void LoadUnload()
    {
        string profilerLibrary = Profiler.GetLocalProfilerLibrary();
        string profilerLibraryCopy = Path.Combine(Path.GetDirectoryName(profilerLibrary), Path.GetFileNameWithoutExtension(profilerLibrary) + "copy" + Path.GetExtension(profilerLibrary));

        Console.WriteLine($"Original lib: {profilerLibrary}");
        Console.WriteLine($"Copied lib: {profilerLibraryCopy}");

        File.Copy(profilerLibrary, profilerLibraryCopy, true);

        LoadUnload(profilerLibraryCopy);

        // Overwrite profiler library
        try {
            File.Copy(profilerLibrary, profilerLibraryCopy, true);
        } catch (Exception e) {
            Console.WriteLine(e);
        }

        // Will segfault
        LoadUnload(profilerLibraryCopy);
    }

    private static void LoadUnload(string library) {
        Console.WriteLine($"Loading '{library}'...");

        // Load profilers library (dlopen on linux)
        NativeLibrary.TryLoad(library, typeof(Segfault).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle);
        Debug.Assert(nint.Zero != handle);

        // Get pointer to method DllGetClassObject (dlsym on linux)
        nint methodHandle = NativeLibrary.GetExport(handle, "DllGetClassObject");
        Debug.Assert(nint.Zero != methodHandle);

        // Cast pointer to DllGetClassObject delegate
        DllGetClassObject dllGetClassObject = Marshal.GetDelegateForFunctionPointer<DllGetClassObject>(methodHandle);
        Debug.Assert(dllGetClassObject != null);

        // Call DllGetClassObject to query the IClassFactory interface that can create instances of Exception Profiler
        dllGetClassObject(ref exceptionProfilerGuid, ref iclassFactoryGuid, out nint classFactoryPtr);
        Debug.Assert(nint.Zero != classFactoryPtr);

        // Since we can't use COM marshalling on Linux, we need to manually get the CreateInstance method pointer from the virtual table
        nint vtablePtr = Marshal.ReadIntPtr(classFactoryPtr);
        nint createInstancePtr = vtablePtr + nint.Size * 3;
        CreateInstance createInstance = Marshal.GetDelegateForFunctionPointer<CreateInstance>(Marshal.ReadIntPtr(createInstancePtr));
        Debug.Assert(createInstance != null);

        // Create instance of profiler, which implements ICoreProfilerCallback8 interface
        createInstance(nint.Zero, nint.Zero, ref iCorProfilerCallback8Guid, out nint ppvObjectPtr);
        Debug.Assert(nint.Zero != ppvObjectPtr);

        // Free library
        NativeLibrary.Free(handle);
    }
}