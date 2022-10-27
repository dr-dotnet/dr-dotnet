using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;

namespace DrDotnet;

public class ProcessDiscovery : IProcessDiscovery
{
    private ILogger _logger;

    public ProcessDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    public async ValueTask<List<Process>> GetDotnetProcessesAsync(Action<float> progressCallback)
    {
        _logger.Log("Listing dotnet processes...");

        var dotnetProcesses = new List<Process>();

        await Task.Run(async () =>
        {
            await Task.Yield();

            var currentProcess = Process.GetCurrentProcess();
            var processes = DiagnosticsClient.GetPublishedProcesses().ToArray();

            for (int i = 0; i < processes.Length; i++)
            {
                progressCallback(1f * i / processes.Length);
                
                try
                {
                    Process process = Process.GetProcessById(processes[i]);

                    _logger.Log($"- [Process] Id: {processes[i]}, Name: {process.ProcessName}");

                    if (processes[i] == currentProcess.Id)
                        continue;

                    dotnetProcesses.Add(process);
                }
                catch(Exception e)
                {
                    _logger.Log("Error listing dotnet processes: " + e.ToString());
                }
            }
            
            _logger.Log("Finished listing dotnet processes.");
        });

        return dotnetProcesses;
    }
}
