using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace DrDotnet.Tests;

public class GCPausesProfilerTests : ProfilerTests
{
    public override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-F785C3186E85}");

    [Test]
    [Order(0)]
    [Timeout(5_000)]
    [NonParallelizable]
    public void Profiler_Exists()
    {
        Assert.NotNull(GetProfiler());
    }

    [Test]
    [Order(1)]
    [Timeout(30_000)]
    [NonParallelizable]
    public async Task Profiler_Counts_Allocations_By_Class()
    {
        ILogger logger = new Logger();
        SessionDiscovery sessionDiscovery = new SessionDiscovery(logger);
        Profiler profiler = GetProfiler();

        Guid sessionId = profiler.StartProfilingSession(Process.GetCurrentProcess().Id, logger);

        // Intentionally allocates memory
        int i = 0;
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
                    GC.Collect();
                }
            }
        });

        var session = await sessionDiscovery.AwaitUntilCompletion(sessionId);

        var summary = session.EnumerateFiles().Where(x => x.Name == "summary.md").FirstOrDefault();
        
        Assert.NotNull(summary, "No summary have been created!");
        
        var content = File.ReadAllText(summary.FullName);
        
        Console.WriteLine(content);
        Console.WriteLine(node.Name);
        
        //Assert.IsTrue(content.Contains("DrDotnet.Tests.TestException:"));
        //Assert.IsFalse(content.Contains("DrDotnet.Tests.TestException: 0"));
    }
}