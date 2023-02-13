using Microsoft.Diagnostics.NETCore.Client;
using System;
using System.Diagnostics;
using System.IO;
using System.Reflection;
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

    /// <summary>
    ///  Name of the profilers library. Different depending on the operating system.
    /// </summary>
    /// <exception cref="NotImplementedException"></exception>
    public static string ProfilerLibraryName => Environment.OSVersion.Platform switch
    {
        PlatformID.Win32NT => "profilers.dll",
        // https://github.com/dotnet/runtime/issues/21660
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.OSX) => "libprofilers.dylib",
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.Linux) => "libprofilers.so",
        _ => throw new NotImplementedException()
    };

    private static string TmpProfilerLibrary;

    /// <summary>
    /// Path of the profilers library in the shared temporary folder
    /// </summary>
    /// <returns></returns>
    public static string GetTmpProfilerLibrary()
    {
        if (TmpProfilerLibrary == null)
        {
            string profilerDll = GetLocalProfilerLibrary();
            string tmpProfilerDll = Path.Combine(PathUtils.DrDotnetBaseDirectory, ProfilerLibraryName);

            // Copy but don't overwrite. Instead, delete before, and copy after. This is required because in Linux if we do
            // a straight override while the library has already been loaded before (and not unloaded), it messed up the mappings 
            // and leads to a segfault
            File.Delete(tmpProfilerDll);
            File.Copy(profilerDll, tmpProfilerDll, false);

            TmpProfilerLibrary = tmpProfilerDll;
        }
        return TmpProfilerLibrary;
    }

    /// <summary>
    /// Path of the profilers library shipped localy with the program
    /// </summary>
    /// <returns></returns>
    public static string GetLocalProfilerLibrary()
    {
        string strExeFilePath = Assembly.GetExecutingAssembly().Location;
        string strWorkPath = Path.GetDirectoryName(strExeFilePath);
        string profilerDll = Path.Combine(strWorkPath, ProfilerLibraryName);
        return profilerDll;
    }

    public Guid StartProfilingSession(int processId, ILogger logger)
    {
        string profilerDll = GetTmpProfilerLibrary();
        var sessionId = Guid.NewGuid();

        logger.LogInformation("Profiler library path: '{profilerDll}'", profilerDll);
        logger.LogInformation("Profiler version: '{version}'", VersionUtils.CurrentVersion);

        DiagnosticsClient client = new DiagnosticsClient(processId);
        
        // try
        // {
        //     var envs = client.GetProcessEnvironment();
        //     logger.LogInformation("Environment variables: " + envs.Count);
        // }
        // catch (Exception e)
        // {
        //     logger.LogError(e, "Can't get process env (IPC broken?)");
        // }
        
        client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

        logger.LogInformation("Attached profiler {ProfilerId} with session {sessionId} to process {processId}", ProfilerId, sessionId, processId);

        return sessionId;
    }
}