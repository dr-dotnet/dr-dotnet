﻿using Moq;
using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;

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
        FileAssert.Exists("libprofilers.so");
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
}