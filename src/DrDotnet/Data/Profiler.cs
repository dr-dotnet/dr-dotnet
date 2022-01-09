using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.IO;
using System.Text;
using System.Threading.Tasks;

namespace DrDotnet
{
    public class Profiler
    {
        public Guid ProfilerId { get; set; }

        public string Name { get; set; }

        public string Description { get; set; }

        public async Task<Session> TryProfileProcess(int processId, ILogger _logger)
        {
            string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
            string strWorkPath = Path.GetDirectoryName(strExeFilePath);
            string profilerDll = Path.Combine(strWorkPath, "profiler.dll");

            var sessionId = Guid.NewGuid();

            DiagnosticsClient client = new DiagnosticsClient(processId);
            client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString()));

            _logger.Log($"Attaching profiler {ProfilerId} with session {sessionId} to process {processId}");

            var sessionFilePath = Session.GetSessionFilePathFromId(sessionId);
            while (!File.Exists(sessionFilePath))
            {
                // Wait until the session manifest has been written
                await Task.Delay(1000);
            }

            return Session.FromPath(sessionFilePath);
        }
    }
}