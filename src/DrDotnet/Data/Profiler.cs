using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.IO;
using System.Text;

namespace DrDotnet;

public class Profiler
{
    public Guid ProfilerId { get; set; }

    public string Name { get; set; }

    public string Description { get; set; }

    private string ProfilerLibraryName => Environment.OSVersion.Platform switch
    {
        PlatformID.Win32NT or
        PlatformID.Win32S or
        PlatformID.Win32Windows or
        PlatformID.WinCE => "profilers.dll",
        PlatformID.Unix => "libprofilers.so",
        PlatformID.MacOSX => "libprofilers.so",
        PlatformID.Other => throw new NotImplementedException(),
        PlatformID.Xbox => throw new NotImplementedException(),
    };

    public Guid StartProfilingSession(int processId, ILogger logger)
    {
        string strExeFilePath = System.Reflection.Assembly.GetExecutingAssembly().Location;
        string strWorkPath = Path.GetDirectoryName(strExeFilePath);
        string profilerDll = Path.Combine(strWorkPath, ProfilerLibraryName);

        var sessionId = Guid.NewGuid();

        DiagnosticsClient client = new DiagnosticsClient(processId);
        client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

        logger.Log($"Attached profiler {ProfilerId} with session {sessionId} to process {processId}");

        return sessionId;
    }
}