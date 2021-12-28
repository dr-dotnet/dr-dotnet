using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Diagnostics;
using System.Linq;

namespace ManagedAttachedNew
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Looking for process...");

            var process = Process.GetProcessesByName("Fibonacci").First();

            Console.WriteLine("Attaching to process...");

            DiagnosticsClient client = new DiagnosticsClient(process.Id);
            var path = "C:\\Users\\oginiaux\\Projects\\traceman\\bin\\Release\\Profiler.Windows.dll";
            Guid guid = new Guid("{805A308B-061C-47F3-9B30-F785C3186E82}");
            client.AttachProfiler(TimeSpan.FromSeconds(10), guid, path, null);

            Console.WriteLine("Attached!");

            Console.ReadLine();
        }
    }
}
