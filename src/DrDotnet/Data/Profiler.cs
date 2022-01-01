using Microsoft.Diagnostics.NETCore.Client;
using System;

namespace DrDotnet
{
    public class Profiler
    {
        public string Name { get; init; }

        public Guid Guid { get; init; }

        public string Description { get; init; }

        public bool TryProfileProcess(int processId)
        {
            string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
            string strWorkPath = System.IO.Path.GetDirectoryName(strExeFilePath);
            string profilerDll = System.IO.Path.Combine(strWorkPath, "profiler.dll");

            DiagnosticsClient client = new DiagnosticsClient(processId);
            client.AttachProfiler(TimeSpan.FromSeconds(10), Guid, profilerDll, null);

            return true;
        }
    }
}