using DrDotnet.Utils;
using Microsoft.Extensions.Logging.Abstractions;
using NUnit.Framework;
using System;
using System.IO;
using System.Runtime.InteropServices;

namespace DrDotnet.Tests;

public class SegfaultReproTests
{
    [Test, Order(1), Explicit]
    [NonParallelizable]
    public void Attach_Using_DiagnosticsClient() {

        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";
        Console.WriteLine(">>> Attach_Using_DiagnosticsClient");



        Console.WriteLine(">>> Copy");

        File.Copy(profilerLibrary, profilerLibraryCopy, true);

        Console.WriteLine(">>> Load");

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Segfault).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle);

        Console.WriteLine(">>> Export");

        nint s = NativeLibrary.GetExport(handle, "DllGetClassObject");

        Console.WriteLine(">>> Free " + s);

        NativeLibrary.Free(handle);
    }

    //[Test, Order(2), Repeat(3)]
    //[NonParallelizable]
    //public void Test()
    //{
    //    string profilerLibrary = "libprofilers.so";
    //    string profilerLibraryCopy = "libprofilerscopy.so";

    //    Console.WriteLine(">> Copy");

    //    File.Copy(profilerLibrary, profilerLibraryCopy, true /* overwrite */);
        
    //    Console.WriteLine(">> Load + Export + Free");
        
    //    Segfault.LoadUnload(NullLogger.Instance, profilerLibraryCopy);
    //}

    [Test]
    public void TestCopy()
    {
        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";

        File.Delete(profilerLibraryCopy);
        File.Copy(profilerLibrary, profilerLibraryCopy, false);

        string check1 = GetMD5Checksum(profilerLibraryCopy);
        
        File.Copy(profilerLibrary, profilerLibraryCopy, true);
        
        string check2 = GetMD5Checksum(profilerLibraryCopy);
        
        Assert.AreEqual(check1, check2);
    }
    
    public static string GetMD5Checksum(string filename)
    {
        using (var md5 = System.Security.Cryptography.MD5.Create())
        {
            using (var stream = System.IO.File.OpenRead(filename))
            {
                var hash = md5.ComputeHash(stream);
                return BitConverter.ToString(hash).Replace("-", "");
            }
        }
    }
}