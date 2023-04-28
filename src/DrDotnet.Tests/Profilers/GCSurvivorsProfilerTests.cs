using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

[StructLayout(LayoutKind.Sequential)]
public record SurvivorObject(int a, int b, long c);

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
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();
        profiler.Parameters.First(x => x.Key == "max_types_display").Value = int.MaxValue.ToString();
        profiler.Parameters.First(x => x.Key == "max_retention_depth").Value = 3.ToString();
        profiler.Parameters.First(x => x.Key == "sort_by_size").Value = false.ToString();

        // Create 1000 SurvivorObject objects that will be placed in the GEN 2 heap
        var survivorObjects = Enumerable.Range(0, 1000).Select(_ => new SurvivorObject(1, 2, 3)).ToArray();

        // Force two garbage collections to promote objects from GEN 0 to GEN 2
        GC.Collect();
        GC.Collect();

        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processDiscovery.GetProcessInfoFromPid(Process.GetCurrentProcess().Id), logger);

        await session.AwaitUntilCompletion();

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "summary.html");

        Assert.NotNull(summary, "No summary have been created!");

        string content = File.ReadAllText(summary.FullName);
        
        string expectedEntry = $"<details><summary><span>({8 /*pointer size in array*/ + 8 /*base size*/ + Marshal.SizeOf<SurvivorObject>() /*object fields size*/},000 bytes) - {survivorObjects.Length}</span>{typeof(SurvivorObject)}</summary>";
        string expectedArrayEntry = $"<details><summary><span>({8 /*pointer size in array*/ + 8 /*base size*/ + Marshal.SizeOf<SurvivorObject>() /*object fields size*/},000 bytes) - {survivorObjects.Length}</span>{typeof(SurvivorObject)}[]</summary>";

        Assert.True(content.Contains(expectedEntry));
        Assert.True(content.Contains(expectedArrayEntry));

        // Check that the objects are in the GEN 2 heap
        Assert.AreEqual(2, GC.GetGeneration(survivorObjects));
    }
}
