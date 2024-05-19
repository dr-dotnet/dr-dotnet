using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class AllocationByClassProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E84}");

    [Test]
    public void Runtime_Version_Is_At_Least_6_0_3()
    {
        // There was a bug introduced in dotnet 6 where ObjectAllocatedByClass callback would not work properly
        // It was fixed in dotnet runtime 6.0.3.
        Assert.GreaterOrEqual(Environment.Version, new Version(6, 0, 3));
    }

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
    [Timeout(30_000)]
    [NonParallelizable]
    public async Task Profiler_Counts_Allocations_By_Class()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        // Intentionally allocates memory
        int i = 0;
        Node node = new Node();
        ThreadPool.QueueUserWorkItem(async _ =>
        {
            while (true)
            {
                node.Child = node = new Node { Name = "mynode" + i++ };
                if (i % 100 == 0)
                {
                    await Task.Delay(10);
                }
                if (i % 1000 == 0)
                {
                    GC.Collect();
                }
            }
        });

        await session.AwaitUntilCompletion();

        var summary = session.EnumerateReports().Where(x => x.Name == "summary.md").FirstOrDefault();

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);

#if DEBUG
        Console.WriteLine(content);
#endif
        Assert.IsTrue(content.Contains("System.String:"));
        Assert.IsTrue(content.Contains("Node:"));
    }

    public class Node
    {
        public string Name;
        public Node Child;
    }
}