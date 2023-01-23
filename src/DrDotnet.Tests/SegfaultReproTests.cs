using NUnit.Framework;
using System;
using System.Runtime.InteropServices;
using System.Threading;
using Microsoft.Diagnostics.NETCore.Client;
using System.Text;
using System.Diagnostics;
using System.Collections.Generic;

namespace DrDotnet.Tests;

public class SegfaultReproTests
{
    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, out nint classFactoryPtr);
    private delegate void CreateInstance(nint self, nint pUnkOuter, ref Guid riid, out nint ppvObjectPtr);

    private static IEnumerable<TestCaseData> GetProfilerPath() {
        yield return new TestCaseData(Profiler.GetLocalProfilerLibrary());
        yield return new TestCaseData(Profiler.GetTmpProfilerLibrary());
    }

    [Test, TestCaseSource(nameof(GetProfilerPath))]
    [Order(1)]
    [NonParallelizable]
    public unsafe void Load_And_Create_Profiler_Instance_Manually(string profilerPath)
    {
        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
        Guid iclassFactoryGuid = new Guid("00000001-0000-0000-c000-000000000046");
        Guid iCorProfilerCallback8Guid = new Guid("5BED9B15-C079-4D47-BFE2-215A140C07E0");

        Console.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Profiler path: {profilerPath}");
        Debug.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Profiler path: {profilerPath}");
        Trace.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Profiler path: {profilerPath}");

        for (int i = 0; i < 3; i++) {
            Console.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Iteration {i}");
            Debug.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Iteration {i}");
            Trace.WriteLine($"[{nameof(Load_And_Create_Profiler_Instance_Manually)}] Iteration {i}");

            // Load profilers library (dlopen on linux)
            Assert.True(NativeLibrary.TryLoad(profilerPath, typeof(SegfaultReproTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
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

    [Test, TestCaseSource(nameof(GetProfilerPath))]
    [Order(2)]
    [NonParallelizable]
    public void Attach_Using_DiagnosticsClient(string profilerPath) {
        Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");

        Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Profiler path: {profilerPath}");
        Debug.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Profiler path: {profilerPath}");
        Trace.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Profiler path: {profilerPath}");

        for (int i = 0; i < 3; i++) {
            Console.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Iteration {i}");
            Debug.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Iteration {i}");
            Trace.WriteLine($"[{nameof(Attach_Using_DiagnosticsClient)}] Iteration {i}");

            int processId = Process.GetCurrentProcess().Id;
            DiagnosticsClient client = new DiagnosticsClient(processId);

            Guid sessionId = Guid.NewGuid();

            Console.Out.Flush();

            client.AttachProfiler(TimeSpan.FromSeconds(10), exceptionProfilerGuid, profilerPath, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

            // This profiler detaches automatically after about 10s
            Thread.Sleep(12_000);
        }
    }
}