using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Utils;

namespace DrDotnet.Tests.Profilers;

public class DuplicatedStringsProfilerTests : ProfilerTests
{
    public override Guid ProfilerGuid => new Guid("{bdaba522-104c-4343-8952-036bed81527d}");

    [Test]
    [Order(0)]
    [Timeout(5_000)]
    [NonParallelizable]
    public void Profiler_Exists()
    {
        Assert.NotNull(GetProfiler());
    }

    [Test, Explicit]
    [Order(1)]
    [Timeout(160_000)]
    [NonParallelizable]
    public async Task Profiler_Lists_Duplicated_Strings()
    {
        Logger logger = new();
        SessionsDiscovery sessionsDiscovery = new(logger);
        ProfilerMetadata profiler = GetProfiler();

        List<string> list = new();
        for (int i = 0; i < 666; i++)
        {
            list.Add(new string('6',6));
        }

        Guid sessionId = ProfilingExtensions.StartProfilingSession(profiler, Process.GetCurrentProcess().Id, logger);

        var session = await sessionsDiscovery.AwaitUntilCompletion(sessionId);

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateFiles().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = File.ReadAllText(summary.FullName);

        Console.WriteLine(content);
    }
}
