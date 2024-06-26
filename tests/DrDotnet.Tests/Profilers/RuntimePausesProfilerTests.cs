using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using DrDotnet.Utils;
using FluentAssertions;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class RuntimePausesProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E85}");

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
    public async Task Profiler_Counts_Runtime_Pauses()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        // Intentionally allocates memory
        int i = 0;
        int collections = 0;
        Node node = new Node();
        ThreadPool.QueueUserWorkItem(async _ =>
        {
            while (true)
            {
                node.Child = node = new Node { Name = "mynode" + i++, List = new List<int>() };
                if (i % 100 == 0)
                {
                    await Task.Delay(10);
                }
                if (i % 1000 == 0)
                {
                    GC.Collect(Random.Shared.Next(1, 5));
                    Interlocked.Increment(ref collections);
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

        content.Should().Contain("Number of pauses:");
        content.Should().NotContain("Number of pauses: 0");
    }
}