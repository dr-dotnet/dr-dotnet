using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;
using DrDotnet.Utils;

namespace DrDotnet.Tests.Profilers;

public class GCSurvivorsProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E86}");

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
    public async Task Profiler_Detects_GC_Survivors_Referenced_From_Gen2()
    {
        Logger logger = new Logger();
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        using var service = new AllocationSimulation(1_000_000, 100_000);
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
