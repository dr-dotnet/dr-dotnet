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

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Segfault).Assembly, DllImportSearchPath.AssemblyDirectory, out nint handle);
        NativeLibrary.TryGetExport(handle, "DllGetClassObject", out nint h1);
        //NativeLibrary.Free(handle);

        Console.WriteLine("Overwrite");
        File.Delete(profilerLibraryCopy);
        File.Copy(profilerLibrary, profilerLibraryCopy, false);

        NativeLibrary.TryLoad(profilerLibraryCopy, typeof(Segfault).Assembly, DllImportSearchPath.AssemblyDirectory, out handle);
        NativeLibrary.TryGetExport(handle, "DllGetClassObject", out nint h2);

        Console.WriteLine("End");
    }
}