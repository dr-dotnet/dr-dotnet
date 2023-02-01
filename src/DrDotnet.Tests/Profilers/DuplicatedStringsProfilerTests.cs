using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;

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
        Profiler profiler = GetProfiler();

        var service1 = new string('6',3);
        var service2 = new string('6',3);
        var service3 = new string('6',3);
        var service4 = new string('6',3);
        var service5 = new string('6',3);
        
        await Task.Delay(1000);
  
        Guid sessionId = profiler.StartProfilingSession(Process.GetCurrentProcess().Id, logger);

        var session = await sessionsDiscovery.AwaitUntilCompletion(sessionId);

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateFiles().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = File.ReadAllText(summary.FullName);

        Console.WriteLine(content);
    }
}
