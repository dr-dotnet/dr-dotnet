using NUnit.Framework;
using System;
using System.Runtime.InteropServices;
using System.Threading;
using Microsoft.Diagnostics.NETCore.Client;
using System.Text;
using System.Diagnostics;

namespace DrDotnet.Tests;

public class SegfaultReproTests
{
    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, out nint classFactoryPtr);
    private delegate void CreateInstance(nint self, nint pUnkOuter, ref Guid riid, out nint ppvObjectPtr);

    [Test]
    [Order(1)]
    [NonParallelizable]
    public unsafe void Load_And_Create_Profiler_Instance_Manually()
    {
        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
        Guid iclassFactoryGuid = new Guid("00000001-0000-0000-c000-000000000046");
        Guid iCorProfilerCallback8Guid = new Guid("5BED9B15-C079-4D47-BFE2-215A140C07E0");

        for (int i = 0; i < 3; i++) {
            Console.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Iteration {i}");

            // Load profilers library (dlopen on linux)
            Assert.True(NativeLibrary.TryLoad("profilers", typeof(SegfaultReproTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
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
    [Order(2)]
    [NonParallelizable]
    public void Attach_Using_DiagnosticsClient()
    {
        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");

        for (int i = 0; i < 3; i++)
        {
            Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Iteration {i}");

            int processId = Process.GetCurrentProcess().Id;
            DiagnosticsClient client = new DiagnosticsClient(processId);

            Guid sessionId = Guid.NewGuid();

            string profilerDll = Profiler.GetLocalProfilerLibrary();

            Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Profiler path: '{profilerDll}'");

            Console.Out.Flush();

            client.AttachProfiler(TimeSpan.FromSeconds(10), exceptionProfilerGuid, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

            // This profiler detaches automatically after about 10s
            Thread.Sleep(12_000);
        }
    }

    [Test]
    [Order(3)]
    [NonParallelizable]
    public void Attach_Using_DiagnosticsClient_TempDir()
    {
        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");

        for (int i = 0; i < 3; i++) {
            Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient_TempDir)}] Iteration {i}");

            int processId = Process.GetCurrentProcess().Id;
            DiagnosticsClient client = new DiagnosticsClient(processId);

            Guid sessionId = Guid.NewGuid();

            string profilerDll = Profiler.GetTmpProfilerLibrary();

            Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Profiler path: '{profilerDll}'");

            Console.Out.Flush();

            client.AttachProfiler(TimeSpan.FromSeconds(10), exceptionProfilerGuid, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

            // This profiler detaches automatically after about 10s
            Thread.Sleep(12_000);
        }
    }
}