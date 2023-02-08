using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Reflection;
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

    private bool TryGetManagedAssemblyNameFromPid(int pid, out string assemblyName, out string version)
    {
        assemblyName = null;
        version = null;
        
        try
        {
            // Todo: Fill a PR on dotnet/diagnostics to open up this API and avoid relying on internal members
            var client = new DiagnosticsClient(pid);
            var methodInfo = typeof(DiagnosticsClient).GetMethod("GetProcessInfo", BindingFlags.Instance | BindingFlags.NonPublic);
            var processInfo = methodInfo.Invoke(client, null);
            var assemblyNameProperty = processInfo.GetType().GetProperty("ManagedEntrypointAssemblyName", BindingFlags.Instance | BindingFlags.Public);
            var clrProductVersionProperty = processInfo.GetType().GetProperty("ClrProductVersionString", BindingFlags.Instance | BindingFlags.Public);
            
            assemblyName = assemblyNameProperty.GetGetMethod().Invoke(processInfo, null) as string;
            version = clrProductVersionProperty.GetGetMethod().Invoke(processInfo, null) as string;

            return true;
        }
        catch (Exception e)
        {
            _logger.LogError(e, "Can't retreive managed assembly name from PID through IPC");
            return false;
        }
    }

    public async ValueTask<List<ProcessInfo>> GetDotnetProcessesAsync(Action<float> progressCallback)
    {
        _logger.LogInformation("Listing dotnet processes...");

        var dotnetProcesses = new List<ProcessInfo>();

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
                    if (processes[i] == currentProcess.Id)
                        continue;
                    
                    _logger.LogInformation($"- [Process] Id: {processes[i]}");
                    
                    if (!TryGetManagedAssemblyNameFromPid(processes[i], out string assemblyName, out string version))
                    {
                        continue;
                    }

                    dotnetProcesses.Add(new ProcessInfo { Pid = processes[i], ManagedAssemblyName = assemblyName, Version = version });
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
