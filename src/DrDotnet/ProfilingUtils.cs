using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;

namespace DrDotnet
{
    public static class ProfilingUtils
    {
        public static readonly Guid DEFAULT_PROFILER = new Guid("{805A308B-061C-47F3-9B30-F785C3186E82}");

        public static bool TryProfileProcess(int processId, Guid profilerGuid)
        {
            string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
            string strWorkPath = System.IO.Path.GetDirectoryName(strExeFilePath);
            string profilerDll = System.IO.Path.Combine(strWorkPath, "profiler.dll");

            Console.WriteLine("Looking for process...");

            var process = Process.GetProcessesByName("Fibonacci").First();

            Logger.Log("Attaching to process...");

            DiagnosticsClient client = new DiagnosticsClient(processId);
            client.AttachProfiler(TimeSpan.FromSeconds(10), profilerGuid, profilerDll, null);

            Logger.Log("Attached!");

            return true;
        }

        public static async Task<List<Process>> GetDotnetProcessesAsync()
        {
            Logger.Log("Listing dotnet processes...");

            var dotnetProcesses = new List<Process>();

            await Task.Run(() =>
            {
                var processes = Process.GetProcesses();
                foreach (var process in processes)
                {
                    try
                    {
                        foreach (ProcessModule pm in process.Modules)
                        {
                            if (pm.ModuleName.StartsWith("coreclr", StringComparison.InvariantCultureIgnoreCase))
                            {
                                Logger.Log($"Dotnet process found: {process.ProcessName}");
                                dotnetProcesses.Add(process);
                                break;
                            }
                        }
                    }
                    catch { }
                }
            });

            return dotnetProcesses;
        }
    }

    public static class Logger
    {
        public static event Action<string> MessageLogged;

        public static void Log(string message)
        {
            var logMessage = $"[{DateTime.Now}] {message}\n";
            MessageLogged?.Invoke(logMessage);
            Console.Write(logMessage);
        }
    }
}