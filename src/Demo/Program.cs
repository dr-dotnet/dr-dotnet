using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;

namespace ManagedAttachedNew;

class Program
{
    static void Main(string[] args) {
        Console.WriteLine("[Demo] Start");

        string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
        string strWorkPath = System.IO.Path.GetDirectoryName(strExeFilePath);

        string profilerDll = Environment.OSVersion.Platform switch {
            PlatformID.Win32NT => "Profiler.Windows.dll",
            PlatformID.Unix => "Profiler.Linux.so",
        };

        profilerDll = System.IO.Path.Combine(strWorkPath, profilerDll);

        Console.WriteLine($"[Demo] Profiler path: {profilerDll}, exists: {File.Exists(profilerDll)}");

        var process = Process.GetCurrentProcess();

        Attach(profilerDll, process);

        Thread.Sleep(5000);

        Attach(profilerDll, process);

        Console.WriteLine("[Demo] Hit key to exit");
        Console.ReadLine();
    }

    private static void Attach(string profilerDll, Process process) {
        Console.WriteLine("[Demo] Attaching...");
        DiagnosticsClient client = new DiagnosticsClient(process.Id);
        Guid guid = new Guid("805A308B-061C-47F3-9B30-F785C3186E82");
        client.AttachProfiler(TimeSpan.FromSeconds(10), guid, profilerDll, null);
        Console.WriteLine("[Demo] Attached!");
    }
}
