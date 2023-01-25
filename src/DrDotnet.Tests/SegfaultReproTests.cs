using DrDotnet.Utils;
using Microsoft.Extensions.Logging.Abstractions;
using NUnit.Framework;
using System;
using System.IO;
using System.Runtime.InteropServices;

namespace DrDotnet.Tests;

public class SegfaultReproTests
{
    [Test, Order(1), Repeat(3)]
    [NonParallelizable]
    public void Attach_Using_DiagnosticsClient() {

        Console.WriteLine(">>> Attach_Using_DiagnosticsClient");

        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";

        Console.WriteLine(">>> Copy");

        File.Copy(profilerLibrary, profilerLibraryCopy, true);

        Console.WriteLine(">>> Load");

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Segfault).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle);

        Console.WriteLine(">>> Export");

        _ = NativeLibrary.GetExport(handle, "DllGetClassObject");

        Console.WriteLine(">>> Free");

        NativeLibrary.Free(handle);
    }

    [Test, Order(2), Repeat(3)]
    [NonParallelizable]
    public void Test()
    {
        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";

        Console.WriteLine(">> Copy");

        File.Copy(profilerLibrary, profilerLibraryCopy, true /* overwrite */);
        
        Console.WriteLine(">> Load + Export + Free");
        
        Segfault.LoadUnload(NullLogger.Instance, profilerLibraryCopy);
    }
}