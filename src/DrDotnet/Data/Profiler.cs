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

    public static string ProfilerLibraryName => Environment.OSVersion.Platform switch
    {
        PlatformID.Win32NT => "profilers.dll",
        // https://github.com/dotnet/runtime/issues/21660
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.OSX) => "libprofilers.dylib",
        PlatformID.Unix when RuntimeInformation.IsOSPlatform(OSPlatform.Linux) => "libprofilers.so",
        _ => throw new NotImplementedException()
    };

    private static string TmpProfilerLibrary;

    public static string GetTmpProfilerLibrary()
    {
        if (TmpProfilerLibrary == null)
        {
            string profilerDll = GetLocalProfilerLibrary();

            string tmpProfilerDll = Path.Combine(PathUtils.DrDotnetBaseDirectory, ProfilerLibraryName);

            // In Linux, there is a segfault if we attach a lib that was attached before but replaced in the meantime
            // This is a problem because we want our profiler library to always be up to date.
            // As a workaround, we rename the profiler library with the version to be sure it's up to date without
            // having to replace it everytime.
            // See https://github.com/dotnet/runtime/issues/80683
            string fileName = Path.GetFileNameWithoutExtension(tmpProfilerDll) + '-' + Assembly.GetEntryAssembly()!.GetName().Version;
            tmpProfilerDll = Path.Combine(Path.GetDirectoryName(tmpProfilerDll)!, fileName + Path.GetExtension(tmpProfilerDll));
            
            // Copy but don't overwrite.
            if (!File.Exists(tmpProfilerDll))
            {
                File.Copy(profilerDll, tmpProfilerDll, false);
            }

            TmpProfilerLibrary = tmpProfilerDll;
        }
        return TmpProfilerLibrary;
    }

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
        
        try
        {
            var process = Process.GetProcessById(processId);
            ArgumentNullException.ThrowIfNull(process);
        }
        catch (Exception e)
        {
            logger.LogError(e, "Process does not seem alive");
            throw;
        }

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