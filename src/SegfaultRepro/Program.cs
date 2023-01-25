using DrDotnet.Utils;
using System.Runtime.InteropServices;

public static class Program
{
    public static void Main()
    {
        Console.WriteLine("Start");

        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";

        File.Copy(profilerLibrary, profilerLibraryCopy, true);

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Program).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle);
        _ = NativeLibrary.GetExport(handle, "DllGetClassObject");
        NativeLibrary.Free(handle);

        Console.WriteLine("Overwrite");
        File.Copy(profilerLibrary, profilerLibraryCopy, true);

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Program).Assembly, DllImportSearchPath.AssemblyDirectory, out handle);
        _ = NativeLibrary.GetExport(handle, "DllGetClassObject");

        Console.WriteLine("End");
    }
}