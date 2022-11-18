using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

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
        _logger.LogInformation("Listing dotnet processes...");

        var dotnetProcesses = new List<Process>();

        // Todo: Use IAsyncEnumerable
        
        try
        {
            var currentProcess = Process.GetCurrentProcess();
            var processes = DiagnosticsClient.GetPublishedProcesses().ToArray();
            
            for (int i = 0; i < processes.Length; i++)
            {
                progressCallback(1f * i / processes.Length);
            
                try
                {
                    Process process = Process.GetProcessById(processes[i]);

                    if (processes[i] == currentProcess.Id)
                        continue;

                    _logger.LogInformation($"- [Process] Id: {processes[i]}, Name: {process.ProcessName}");

                    _logger.LogDebug($"  - Main module name: {process.MainModule.ModuleName}, File: {process.MainModule.FileName}, Main window title: {process.MainWindowTitle}, Site name: {process.Site?.Name}");
                
                    foreach (ProcessModule module in process.Modules)
                    {
                        _logger.LogDebug($"  - Module name: {module.ModuleName}, File: {module.FileName}, Site: {module.Site?.Name}");
                    }

                    dotnetProcesses.Add(process);
                }
                catch (Exception e)
                {
                    _logger.LogError(e, "Can't read process {ProcessId} information.", processes[i]);
                }
            }
        
            _logger.LogInformation("Finished listing dotnet processes.");
        }
        catch (Exception e)
        {
            _logger.LogError(e, "Failed listing dotnet processes.");
        }

        return dotnetProcesses;
    }
}
