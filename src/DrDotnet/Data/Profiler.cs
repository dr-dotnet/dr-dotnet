using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Extensions.Logging;

namespace DrDotnet;

public class Profiler
{
    public Guid ProfilerId { get; set; }

    public string Name { get; set; }

    public string Description { get; set; }

    private string ProfilerLibraryName => Environment.OSVersion.Platform switch
    {
        PlatformID.Win32NT => "profilers.dll",
        // https://github.com/dotnet/runtime/issues/21660
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.OSX)  => "libprofilers.dylib",
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.Linux)  => "libprofilers.so",
        _ => throw new NotImplementedException()
    };

    public Guid StartProfilingSession(int processId, ILogger logger)
    {
        string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
        string strWorkPath = Path.GetDirectoryName(strExeFilePath);
        string profilerDll = Path.Combine(strWorkPath, ProfilerLibraryName);
        var sessionId = Guid.NewGuid();

        // Copy DLL for sidecar profiling through shared volume /tmp
        // Could be improved
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
        {
            try
            {
                string tmpProfilerDll = Path.Combine("/tmp", ProfilerLibraryName);
                File.Copy(profilerDll, tmpProfilerDll, true);
                profilerDll = tmpProfilerDll;
                logger.LogInformation("Profiler lib copied to {profilerDll}", profilerDll);
            }
            catch (Exception e)
            {
                logger.LogError(e, "Error while copying profilers library");
            }
        }

        DiagnosticsClient client = new DiagnosticsClient(processId);

        client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

        logger.LogInformation("Attached profiler {ProfilerId} with session {sessionId} to process {processId}", ProfilerId, sessionId, processId);

        return sessionId;
    }
}