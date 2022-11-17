using Microsoft.Diagnostics.NETCore.Client;
using Microsoft.Diagnostics.Tracing;
using Microsoft.Diagnostics.Tracing.Parsers;
using System;
using System.Collections.Generic;
using System.Diagnostics.Tracing;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

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

        logger.Log($"Profiler DLL path is '{profilerDll}'. Exists: {File.Exists(profilerDll)}");

        DiagnosticsClient client = new DiagnosticsClient(processId);

        try {
            var env = client.GetProcessEnvironment();
            foreach (var pair in env) {
                logger.Log($"- Env name: {pair.Key}, value: {pair.Value}");
            }
        } catch(Exception e) {
            logger.Log($"[ERROR] Error while retreiving env variables: {e}");
        }

        try {
            var providers = new List<EventPipeProvider>()
            {
                new EventPipeProvider("Microsoft-Windows-DotNETRuntime",
                    EventLevel.Informational, (long)ClrTraceEventParser.Keywords.GC)
            };
            using (var session = client.StartEventPipeSession(providers, false)) {
                var source = new EventPipeEventSource(session.EventStream);

                source.Clr.All += (TraceEvent obj) => {
                    logger.Log("ETW EVENT: " + obj.EventName);
                };

                try {
                    source.Process();
                }
                // NOTE: This exception does not currently exist. It is something that needs to be added to TraceEvent.
                catch (Exception e) {
                    logger.Log("Error encountered while processing events");
                    logger.Log(e.ToString());
                }
            }
        }
        catch (Exception e) {
            logger.Log($"[ERROR] Error while retreiving etw runtime events: {e}");
        }

        client.AttachProfiler(TimeSpan.FromSeconds(10), ProfilerId, profilerDll, Encoding.UTF8.GetBytes(sessionId.ToString() + "\0"));

        logger.Log($"Attached profiler {ProfilerId} with session {sessionId} to process {processId}");

        return sessionId;
    }
}