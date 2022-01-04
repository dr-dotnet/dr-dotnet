using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.IO;
using System.Text;
using System.Threading.Tasks;

namespace DrDotnet
{
    public class Profiler
    {
        public string Name { get; init; }

        public Guid Guid { get; init; }

        public string Description { get; init; }

        public async Task<AnalysisData> TryProfileProcess(int processId, ILogger _logger)
        {
            string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
            string strWorkPath = Path.GetDirectoryName(strExeFilePath);
            string profilerDll = Path.Combine(strWorkPath, "profiler.dll");

            var session_guid = Guid.NewGuid();

            DiagnosticsClient client = new DiagnosticsClient(processId);
            client.AttachProfiler(TimeSpan.FromSeconds(10), Guid, profilerDll, Encoding.UTF8.GetBytes(session_guid.ToString()));

            _logger.Log($"Attaching profiler {Guid} with session {session_guid}");

            while (!File.Exists($"/dr-dotnet/{session_guid}.json"))
            {
                await Task.Delay(1000);
            }

            return new AnalysisData();
        }
    }
}