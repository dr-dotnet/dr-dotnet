using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;

namespace DrDotnet;

public class ProcessDiscovery : IProcessDiscovery
{
    private ILogger _logger;

    public ProcessDiscovery(ILogger logger)
    {
        _logger = logger;
    }

    private List<Process> _processes;

    public async ValueTask<List<Process>> GetDotnetProcessesAsync(Action<float> progressCallback)
    {
        if (_processes != null)
            return _processes;

        _logger.Log("Listing dotnet processes...");

        var dotnetProcesses = new List<Process>();

        await Task.Run(async () =>
        {
            await Task.Yield();

            var processes = Process.GetProcesses();
            for (int i = 0; i < processes.Length; i++)
            {
                progressCallback(1f * i / processes.Length);

                _logger.Log($"- [Process] Id: {processes[i].Id}, Name: {processes[i].ProcessName}");
                
                if (processes[i].ProcessName.StartsWith("DrDotnet"))
                    continue;
                
                try
                {
                    foreach (ProcessModule pm in processes[i].Modules)
                    {
                        _logger.Log($"  - [Module] Name: {pm.ModuleName}, File: {pm.FileName}");
                        
                        if (pm.ModuleName.StartsWith("coreclr", StringComparison.InvariantCultureIgnoreCase))
                        {
                            _logger.Log($"Dotnet process found: {processes[i].ProcessName}");
                            dotnetProcesses.Add(processes[i]);
                            break;
                        }
                    }
                }
                catch(Exception e)
                {
                    _logger.Log("Error listing dotnet processes: " + e.ToString());
                }
            }
            
            _logger.Log("Finished listing dotnet processes.");
        });

        return _processes = dotnetProcesses;
    }
}
