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

    [Test]
    public void Can_Load_CreateInstance_And_Free_Library() {
        for (int i = 0; i < 3; i++) {
            Assert.True(NativeLibrary.TryLoad("profilers", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle));
            Assert.AreNotEqual(nint.Zero, handle);
            nint methodHandle;
            Assert.AreNotEqual(nint.Zero, methodHandle = NativeLibrary.GetExport(handle, "DllGetClassObject"));
            DllGetClassObject dllGetClassObject = Marshal.GetDelegateForFunctionPointer<DllGetClassObject>(methodHandle);
            Assert.NotNull(dllGetClassObject);
            Guid exceptionProfilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
            Guid iclassFactoryGuid = new Guid("00000001-0000-0000-c000-000000000046");
            Guid iCorProfilerCallback8Guid = new Guid("5BED9B15-C079-4D47-BFE2-215A140C07E0");
            // Get IClassFactory that can create instances of Exception Profiler
            dllGetClassObject(ref exceptionProfilerGuid, ref iclassFactoryGuid, out IClassFactory classFactory);
            // Create instance of profiler, which implements ICoreProfilerCallback8 interface
            classFactory.CreateInstance(null, ref iCorProfilerCallback8Guid, out object ppvObject);
            Assert.NotNull(ppvObject);
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

    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, [Out, MarshalAs(UnmanagedType.Interface)] out IClassFactory classFactory);

    [Guid("00000001-0000-0000-c000-000000000046")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    internal interface IClassFactory
    {
        void CreateInstance([MarshalAs(UnmanagedType.IUnknown)] object pUnkOuter, ref Guid riid, [MarshalAs(UnmanagedType.IUnknown)] out object ppvObject);
        void LockServer(bool fLock);
    }

    [Test]
    public void Profilers_Are_Discovered()
    {
        ProfilersDiscovery profilersDiscovery = new (Mock.Of<ILogger>());
        List<Profiler> profilers = profilersDiscovery.GetProfilers(true);
        Assert.IsNotEmpty(profilers);
    }
}
