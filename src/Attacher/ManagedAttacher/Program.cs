using Profiler;
using System;
using System.Diagnostics;

namespace ProfilerAttacher
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");

            CLRMetaHost metahost = new CLRMetaHost();

            //foreach (var installedRuntime in metahost.EnumerateInstalledRuntimes())
            //{
            //    Console.WriteLine("Installed: " + installedRuntime.GetVersionString());
            //}

            CLRRuntimeInfo highestLoadedRuntime = null;
            foreach (CLRRuntimeInfo runtime in metahost.EnumerateInstalledRuntimes())
            {
                Console.WriteLine("Installed: " + runtime.GetVersionString());

                if (highestLoadedRuntime == null || string.Compare(highestLoadedRuntime.GetVersionString(), runtime.GetVersionString(), StringComparison.OrdinalIgnoreCase) < 0)
                {
                    highestLoadedRuntime = runtime;
                }
            }

            Guid profilerGuid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");

            highestLoadedRuntime.GetProfilingInterface()
                .AttachProfiler(30436, 10000, ref profilerGuid, "C:\\Users\\oginiaux\\Projects\\DotNextMoscow2019\\x64\\Debug\\DotNext.Profiler.Windows.dll", IntPtr.Zero, 0);

            Console.ReadKey();
        }
    }
}