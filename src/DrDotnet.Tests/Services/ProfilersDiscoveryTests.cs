using Moq;
using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using Microsoft.Extensions.Logging;

namespace DrDotnet.Tests.Services;

public class ProfilersDiscoveryTests
{
    [Test]
    [Platform("Win")]
    public void Profilers_Libary_Is_Present_Windows()
    {
        Console.WriteLine(Directory.GetCurrentDirectory());
        FileAssert.Exists("profilers.dll");
    }

    [Test]
    [Platform("Linux")]
    public void Profilers_Libary_Is_Present_Linux()
    {
        Console.WriteLine(Directory.GetCurrentDirectory());
        FileAssert.Exists("libprofilers.so", Directory.GetCurrentDirectory());
    }
    
    [Test]
    [Platform("MacOsX")]
    public void Profilers_Libary_Is_Present_MacOS()
    {
        Console.WriteLine(Directory.GetCurrentDirectory());
        FileAssert.Exists("libprofilers.dylib", Directory.GetCurrentDirectory());
    }

    [Test]
    public void Profilers_Are_Discovered() {
        ProfilersDiscovery profilersDiscovery = new(Mock.Of<ILogger>());
        List<Profiler> profilers = profilersDiscovery.GetProfilers(true);
        Assert.IsNotEmpty(profilers);
    }

    [Test]
    public void Can_Load_Library() {
        for (int i = 0; i < 3; i++) {
            Assert.True(NativeLibrary.TryLoad("profilers", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
            Assert.AreNotEqual(nint.Zero, handle);
            Assert.AreNotEqual(nint.Zero, NativeLibrary.GetExport(handle, "DllGetClassObject"));
        }
    }

    [Test]
    public void Can_Load_And_Free_Library() {
        for (int i = 0; i < 3; i++) {
            Assert.True(NativeLibrary.TryLoad("profilers", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
            Assert.AreNotEqual(nint.Zero, handle);
            Assert.AreNotEqual(nint.Zero, NativeLibrary.GetExport(handle, "DllGetClassObject"));
            NativeLibrary.Free(handle);
        }
    }

    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, [Out] out nint classFactoryPtr);
    private delegate void CreateInstance(nint self, nint pUnkOuter, ref Guid riid, out nint ppvObjectPtr);

    [Test]
    public unsafe void Can_Load_CreateInstance_And_Free_Library() {

        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
        Guid iclassFactoryGuid = new Guid("00000001-0000-0000-c000-000000000046");
        Guid iCorProfilerCallback8Guid = new Guid("5BED9B15-C079-4D47-BFE2-215A140C07E0");

        for (int i = 0; i < 3; i++)
        {
            Console.WriteLine($"[{nameof(Can_Load_CreateInstance_And_Free_Library)}] Iteration {i}");

            // Load profilers library (dlopen on linux)
            Assert.True(NativeLibrary.TryLoad("profilers", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
            Assert.AreNotEqual(nint.Zero, handle);

            // Get pointer to method DllGetClassObject (dlsym on linux)
            nint methodHandle = NativeLibrary.GetExport(handle, "DllGetClassObject");
            Assert.AreNotEqual(nint.Zero, methodHandle);

            // Cast pointer to DllGetClassObject delegate
            DllGetClassObject dllGetClassObject = Marshal.GetDelegateForFunctionPointer<DllGetClassObject>(methodHandle);
            Assert.NotNull(dllGetClassObject);

            // Call DllGetClassObject to query the IClassFactory interface that can create instances of Exception Profiler
            dllGetClassObject(ref exceptionProfilerGuid, ref iclassFactoryGuid, out nint classFactoryPtr);
            Assert.AreNotEqual(nint.Zero, classFactoryPtr);

            // Since we can't use COM marshalling on Linux, we need to manually get the CreateInstance method pointer from the virtual table
            nint vtablePtr = Marshal.ReadIntPtr(classFactoryPtr);
            CreateInstance createInstance = Marshal.GetDelegateForFunctionPointer<CreateInstance>(Marshal.ReadIntPtr(vtablePtr + nint.Size * 3));
            Assert.NotNull(createInstance);

            // Create instance of profiler, which implements ICoreProfilerCallback8 interface
            createInstance(classFactoryPtr, nint.Zero, ref iCorProfilerCallback8Guid, out nint ppvObjectPtr);
            Assert.AreNotEqual(nint.Zero, ppvObjectPtr);

            // Free library
            NativeLibrary.Free(handle);
        }
    }

    [Test]
    [Platform("Linux")]
    public void Can_Load_More_Than_Once_Different_Path() {
        Directory.CreateDirectory("tmp");
        File.Copy("libprofilers.so", "tmp/libprofilers.so");
        for (int i = 0; i < 3; i++) {
            Assert.True(NativeLibrary.TryLoad("tmp/libprofilers.so", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
            Assert.AreNotEqual(nint.Zero, handle);
            Assert.AreNotEqual(nint.Zero, NativeLibrary.GetExport(handle, "DllGetClassObject"));
            NativeLibrary.Free(handle);
        }
    }
}