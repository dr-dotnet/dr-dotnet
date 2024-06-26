using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace DrDotnet.Tests.Profilers;

public class MemoryLeakProfilerTests : ProfilerTests
{
    protected override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E83}");

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
    public async Task Profiler_Detects_Memory_Leaks()
    {
        ILogger<ProcessDiscovery> logger = NullLogger<ProcessDiscovery>.Instance;
        ProcessDiscovery processDiscovery = new ProcessDiscovery(logger);
        ProfilerInfo profiler = GetProfiler();

        Assert.True(processDiscovery.TryGetProcessInfoFromPid(Process.GetCurrentProcess().Id, out ProcessInfo? processInfo), "Could not find current process info");
        SessionInfo session = ProfilingExtensions.StartProfilingSession(profiler, processInfo, logger);

        // Intentionally allocates memory
        int i = 0;
        Node node = new Node();
        var baseNode = node;
        ThreadPool.QueueUserWorkItem(async _ =>
        {
            while (true)
            {
                node.Child = node = new Node { Name = "mynode" + i++, List = new List<int>() };
                if (i % 100 == 0)
                {
                    await Task.Delay(10);
                }
                if (i % 5000 == 0)
                {
                    GC.Collect(2, GCCollectionMode.Forced, blocking: true, compacting: false);
                }
            }
        });

        // Warmup
        await Task.Delay(1000);

        await session.AwaitUntilCompletion();

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateReports().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = await File.ReadAllTextAsync(summary.FullName);

#if DEBUG
        Console.WriteLine(content);
#endif
        
        Console.WriteLine(node.Name);
        Console.WriteLine(baseNode.Name);

        // TODO: Assert on results
    }
}
    
public class Node
{
    public string Name;
    public Node Child;
    public List<int> List;
}