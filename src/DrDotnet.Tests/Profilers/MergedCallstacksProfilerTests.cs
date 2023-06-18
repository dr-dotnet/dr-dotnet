using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class MergedCallstacksProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{9404d16c-b49e-11ed-afa1-0242ac120002}");

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
    public async Task Profiler_Merged_Callstacks()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        using var service1 = new PrimeSimulation();
        using var service2 = new PrimeSimulation();
        using var service3 = new PrimeSimulation();
        using var service4 = new PrimeSimulation();
        
        await Task.Delay(3000);
  
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processDiscovery.GetProcessInfoFromPid(Process.GetCurrentProcess().Id), logger);

        await session.AwaitUntilCompletion();

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = File.ReadAllText(summary.FullName);

        Console.WriteLine(content);
    }
}
