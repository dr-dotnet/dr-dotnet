using Moq;
using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using Microsoft.Extensions.Logging;

namespace DrDotnet.Tests.Services;

public class ProfilersDiscoveryTests {
    [Test]
    [Platform("Win")]
    public void Profilers_Libary_Is_Present_Windows() {
        Console.WriteLine(Directory.GetCurrentDirectory());
        FileAssert.Exists("profilers.dll");
    }

    [Test]
    [Platform("Linux")]
    public void Profilers_Libary_Is_Present_Linux() {
        Console.WriteLine(Directory.GetCurrentDirectory());
        FileAssert.Exists("libprofilers.so", Directory.GetCurrentDirectory());
    }

    [Test]
    [Platform("MacOsX")]
    public void Profilers_Libary_Is_Present_MacOS() {
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