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
    public void Can_Load_Library()
    {
        if (NativeLibrary.TryLoad("profilers", typeof(ProfilersDiscoveryTests).Assembly, DllImportSearchPath.AssemblyDirectory, out IntPtr handle))
        {
            NativeLibrary.Free(handle);
        }
        else
        {
            Assert.Fail();
        }
    }

    [Test]
    public void Profilers_Are_Discovered()
    {
        ProfilersDiscovery profilersDiscovery = new (Mock.Of<ILogger>());
        List<Profiler> profilers = profilersDiscovery.GetProfilers(true);
        Assert.IsNotEmpty(profilers);
    }

    [DllImport("libdl")]
    protected static extern IntPtr dlopen(string filename, int flags);

    [DllImport("libdl")]
    protected static extern IntPtr dlsym(IntPtr handle, string symbol);

    [Test]
    [Platform("Linux")]
    public void Segfault_Repro() {
        Console.WriteLine($"[SegfaultRepro] Start");
        const int RTLD_NOW = 2; // for dlopen's flags 
        IntPtr moduleHandle = dlopen("profiler", RTLD_NOW);
        Console.WriteLine($"[SegfaultRepro] Module Handle: {moduleHandle}");
        IntPtr ptr = dlsym(moduleHandle, "DllGetClassObject");
        Console.WriteLine($"[SegfaultRepro] MethodHandle: {ptr}");
        DllGetClassObject func = Marshal.GetDelegateForFunctionPointer(ptr, typeof(DllGetClassObject)) as DllGetClassObject;
        Console.WriteLine($"[SegfaultRepro] DllGetClassObject: {func}");
    }

    private delegate int DllGetClassObject(ref Guid clsid, ref Guid iid, [Out, MarshalAs(UnmanagedType.Interface)] out IClassFactory classFactory);

    [Guid("00000001-0000-0000-c000-000000000046")]
    [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    [ComImport]
    internal interface IClassFactory
    {
        void CreateInstance([MarshalAs(UnmanagedType.IUnknown)] object pUnkOuter, ref Guid riid, [MarshalAs(UnmanagedType.IUnknown)] out object ppvObject);
        void LockServer(bool fLock);
    }
}
