using DrDotnet.Utils;
using Microsoft.Extensions.Logging.Abstractions;

public static class Program
{
    public static void Main() {
        string profilerLibrary = "libprofilers.so";
        string profilerLibraryCopy = "libprofilerscopy.so";

        File.Copy(profilerLibrary, profilerLibraryCopy, true /* overwrite */);

        Console.WriteLine(">> Load + Export");

        Segfault.LoadUnload(NullLogger.Instance, profilerLibraryCopy);

        Console.WriteLine(">> Copy");

        // Without this, there is no segfault
        File.Copy(profilerLibrary, profilerLibraryCopy, true /* overwrite */);

        Console.WriteLine(">> Load + Export");

        Segfault.LoadUnload(NullLogger.Instance, profilerLibraryCopy);

        Console.WriteLine(">> Done");

    }
}