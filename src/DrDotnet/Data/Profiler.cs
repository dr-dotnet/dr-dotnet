using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Extensions.Logging;
using DrDotnet.Utils;

namespace DrDotnet;

public class Profiler
{
    public Guid ProfilerId { get; set; }

    public string Name { get; set; }

    public string Description { get; set; }

    public int PIndex = 0;
    
    public static string ProfilerLibraryName => Environment.OSVersion.Platform switch
    {
        PlatformID.Win32NT => $"profilers.dll",
        // https://github.com/dotnet/runtime/issues/21660
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.OSX) => $"libprofilers.dylib",
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.Linux) => $"libprofilers.so",
        _ => throw new NotImplementedException()
    };

    public static string GetTmpProfilerLibrary()
    {
        string profilerDll = GetLocalProfilerLibrary();

        string tmpProfilerDll = Path.Combine(PathUtils.DrDotnetBaseDirectory, ProfilerLibraryName);
        Exception ex = null;
        try {
            File.Copy(profilerDll, tmpProfilerDll, true);
        }
        catch (Exception e) {
            ex = e;
        }
        profilerDll = tmpProfilerDll;

        if (!File.Exists(profilerDll)) {
            throw new FileNotFoundException("Profiler library not found", profilerDll, ex);
        }

        return profilerDll;
    }

    public static string GetLocalProfilerLibrary()
    {
        string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
        string strWorkPath = Path.GetDirectoryName(strExeFilePath);
        string profilerDll = Path.Combine(strWorkPath, ProfilerLibraryName);
        return profilerDll;
    }

    public Guid StartProfilingSession(int processId, ILogger logger)
    {
        PIndex++;
        
        string profilerDll = GetTmpProfilerLibrary();
        var sessionId = Guid.NewGuid();

        try
        {
            var process = Process.GetProcessById(processId);
            ArgumentNullException.ThrowIfNull(process);
        }
        catch (Exception e)
        {
            logger.LogError(e, "Process does not seem alive");
        }

        DiagnosticsClient client = new DiagnosticsClient(processId);
        
        try
        {
            var envs = client.GetProcessEnvironment();
            logger.LogInformation("Environment variables: " + envs.Count);
        }
        catch (Exception e)
        {
            logger.LogError(e, "Can't get process env (IPC broken?)");
        }
        
        client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

        logger.LogInformation("Attached profiler {ProfilerId} with session {sessionId} to process {processId}", ProfilerId, sessionId, processId);

        return sessionId;
    }
}