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
            string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
            string strWorkPath = System.IO.Path.GetDirectoryName(strExeFilePath);
            string profilerDll = System.IO.Path.Combine(strWorkPath, "Profiler.Windows.dll");

            Console.WriteLine("Looking for process...");

            var process = Process.GetProcessesByName("Fibonacci").First();

            Console.WriteLine("Attaching to process...");

            DiagnosticsClient client = new DiagnosticsClient(process.Id);
            Guid guid = new Guid("{805A308B-061C-47F3-9B30-F785C3186E82}");
            client.AttachProfiler(TimeSpan.FromSeconds(10), guid, profilerDll, null);

            Console.WriteLine("Attached!");

            Console.ReadLine();
        }
    }
}
